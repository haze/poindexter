use std::cell::RefCell;

struct Line {
    Text: String,
    Handler: Box<std::io::Write>,
}

pub struct PrintWorker {
    work_character: String,
    last_message: RefCell<Option<Line>>,
    working: RefCell<bool>,
}

impl std::default::Default for PrintWorker {
    fn default() -> Self {
        PrintWorker {
            work_character: String::from("\u{2026}"),
            last_message: RefCell::new(None),
            working: RefCell::new(false),
        }
    }
}

impl PrintWorker {
    pub fn println<S: Into<String>>(&self, text: S) -> Result<(), std::io::Error> {
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();

        // if we are not working, now we are.
        let working = *self.working.borrow();
        if !working {
            self.set_work_state(true);
            self.println_working_to(&mut handle, text)
        } else {
            self.stop_working(); // stop working (remove ...)
                                 // print final statement
                                 // // start printing new one
            self.println_working_to(&mut handle, text)
        }
    }

    fn reset_cursor<W: std::io::Write>(&self, sink: &mut W) -> Result<(), std::io::Error> {
        // clear if we can
        let mut buf = [0; 1];
        let result = '\r'.encode_utf8(&mut buf);
        if let Some(ref last_message_text) = *self.last_message.borrow() {
            let clear_amount = last_message_text.len() + self.work_character.len();
            sink.write_all(result.as_bytes())?;
            sink.write_all(
                &std::iter::repeat(' ' as u8)
                    .take(clear_amount)
                    .collect::<Vec<u8>>()
                    .as_slice(),
            )?;
            sink.flush()?;
            dbg!(&result);
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
        text.push_str(&*self.work_character);
        let text = text; // ensure we cannot mutate after this point
        sink.write_all(text.as_bytes())?;
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

    pub fn stop_working(&self) -> Result<(), std::io::Error> {
        self.set_work_state(false);
        match *self.last_message.borrow() {
            Some(ref last_message_text) => {
                self.reset_cursor(&mut handle)?;
                self.println_to(&mut handle, last_message_text)
            }
            None => Ok(()),
        }
    }

    fn set_work_state(&self, state: bool) {
        *self.working.borrow_mut() = state;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
