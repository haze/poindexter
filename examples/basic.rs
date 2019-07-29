use poindexter::PrintWorker;
use std::{thread, time};

fn main() -> Result<(), std::io::Error> {
    let w = PrintWorker::default();
    w.println("this is a test string")?;
    thread::sleep(time::Duration::from_secs(5));
    w.println(format!("allocated strings work as well"))?;
    thread::sleep(time::Duration::from_secs(5));
    w.println("done")
}
