use std::cell::RefCell;

mod test;

pub struct PrintWorker {
    // what to display when work is being done
    work_suffix: String,

    // what to suffix the finalized work string with
    finish_suffix: String,

    // last message sent through the print worker
    last_message: RefCell<Option<String>>,

    // if we have used the stdout short functions (for drop impl)
    stdout_used: RefCell<bool>,
}

impl std::default::Default for PrintWorker {
    fn default() -> Self {
        PrintWorker {
            work_suffix: String::from("\u{2026}"),
            finish_suffix: String::new(),
            last_message: RefCell::new(None),
            stdout_used: RefCell::new(false),
        }
    }
}

impl Drop for PrintWorker {
    fn drop(&mut self) {
        if *self.stdout_used.borrow() {
            // clear work on stdout
            let stdout = std::io::stdout();
            let mut handle = stdout.lock();
            self.finish_work(&mut handle).ok();
        }
    }
}

impl PrintWorker {
    fn finish_work<W: std::io::Write>(&self, sink: &mut W) -> Result<(), std::io::Error> {
        // reprint last message
        match *self.last_message.borrow() {
            Some(ref last_message_text) => {
                self.reset_cursor(sink)?;
                self.println_to(sink, last_message_text)
            }
            None => Ok(()),
        }
    }

    pub fn println<S: Into<String>>(&self, text: S) -> Result<(), std::io::Error> {
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        self.easy_println_to(&mut handle, text)
    }

    fn easy_println_to<S: Into<String>, W: std::io::Write>(
        &self,
        sink: &mut W,
        text: S,
    ) -> Result<(), std::io::Error> {
        // im at a fault here, is it easier to just set stdout_used each time
        // or is it okay to make a check? in theory the check is 2 ops
        // while just setting is 1, but idk
        *self.stdout_used.borrow_mut() = true;
        if (*self.last_message.borrow()).is_some() {
            self.finish_work(sink)?;
        }
        self.println_working_to(sink, text)
    }

    fn reset_cursor<W: std::io::Write>(&self, sink: &mut W) -> Result<(), std::io::Error> {
        // clear if we can
        let mut buf = [0; 1];
        let result = '\r'.encode_utf8(&mut buf);
        if let Some(ref last_message_text) = *self.last_message.borrow() {
            let clear_amount = last_message_text.len() + self.work_suffix.len();
            sink.write_all(result.as_bytes())?;
            sink.write_all(
                &std::iter::repeat(b' ')
                    .take(clear_amount)
                    .collect::<Vec<u8>>()
                    .as_slice(),
            )?;
            sink.flush()?;
        }
        sink.write_all(result.as_bytes())?;
        sink.flush()
    }

    pub fn println_working_to<S: Into<String>, W: std::io::Write>(
        &self,
        sink: &mut W,
        text: S,
    ) -> Result<(), std::io::Error> {
        // investigate this into
        let mut text = text.into();
        let original_text_length = text.len();
        text.push_str(&*self.work_suffix);
        sink.write_all(text.as_bytes())?;
        // restore to original text for saving
        text.truncate(original_text_length);
        *self.last_message.borrow_mut() = Some(text);
        sink.flush()
    }

    pub fn println_to<S: Into<String>, W: std::io::Write>(
        &self,
        sink: &mut W,
        text: S,
    ) -> Result<(), std::io::Error> {
        // investigate this into
        let mut text = text.into();
        text.push('\n');
        let text = text; // ensure we cannot mutate after this point
        sink.write_all(text.as_bytes())?;
        sink.flush()
    }
}
