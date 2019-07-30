use crate::PrintWorker;
use std::io::Cursor;

type ByteCursor = Cursor<Vec<u8>>;

// Attempts to use a single allocation for the cursor's internal buffer
fn cursor_for(lines: &Vec<String>) -> ByteCursor {
    let total_capacity = lines.iter().map(String::capacity).sum();
    Cursor::new(Vec::with_capacity(total_capacity))
}

// convert a vec of &str to String
fn lines(lines: Vec<&str>) -> Vec<String> {
    lines.iter().cloned().map(String::from).collect()
}

// get_lines_from_cursor returns a simple vector of strings split by newline
// from the given bytecursor (todo(hazebooth): turn  ByteCursor into a tuplestruct & impl?)
fn get_lines_from_cursor(cursor: &ByteCursor) -> Result<Vec<&str>, String> {
    Ok(std::str::from_utf8(cursor.get_ref())
        .map_err(|e| format!("Failed to encode lines from ByteCursor: {}", e))?
        .split("\n")
        .collect())
}

// reset_str constructs a raw/literal representation of the ascii characters
// used to clear the terminal. this is usually
// {loading_msg}{work_sufic\r<space>\r{loading_msg}
fn reset_str(source: &str, work_suffix: &str) -> String {
    let source_len = source.len() + work_suffix.len();
    let spaces: String = std::iter::repeat(' ').take(source_len).collect();
    let work = format!("{}{}", source, work_suffix);
    let reset = format!("\r{}\r", spaces);
    let result = format!("{}{}{}", work, reset, source);
    result
}

#[test]
fn simple_wait_works() -> Result<(), String> {
    let src_lines = lines(vec!["First Line", "Second Line", "Third Line"]);
    let mut mock_stdin = cursor_for(&src_lines);
    let writer = PrintWorker::default();

    // manual loop unroll
    writer
        .easy_println_to(&mut mock_stdin, &src_lines[0])
        .map_err(|e| format!("Failed to write to cursor: {}", e))?;
    let lines = get_lines_from_cursor(&mock_stdin)?;
    assert_eq!(&lines[0], &"First Line…");

    writer
        .easy_println_to(&mut mock_stdin, &src_lines[1])
        .map_err(|e| format!("Failed to write to cursor: {}", e))?;
    let lines = get_lines_from_cursor(&mock_stdin)?;
    assert_eq!(&lines[0], &reset_str(&src_lines[0], &"…"));
    assert_eq!(&lines[1], &"Second Line…");

    writer
        .easy_println_to(&mut mock_stdin, &src_lines[2])
        .map_err(|e| format!("Failed to write to cursor: {}", e))?;
    let lines = get_lines_from_cursor(&mock_stdin)?;
    assert_eq!(&lines[0], &reset_str(&src_lines[0], &"…"));
    assert_eq!(&lines[1], &reset_str(&src_lines[1], &"…"));
    assert_eq!(&lines[2], &"Third Line…");
    writer.finish_work(&mut mock_stdin);
    let lines = get_lines_from_cursor(&mock_stdin)?;
    assert_eq!(&lines[0], &reset_str(&src_lines[0], &"…"));
    assert_eq!(&lines[1], &reset_str(&src_lines[1], &"…"));
    assert_eq!(&lines[2], &reset_str(&src_lines[2], &"…"));
    assert_eq!(&lines[3], &"");
    Ok(())
}
