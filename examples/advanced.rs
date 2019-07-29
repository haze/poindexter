use poindexter::PrintWorker;
use std::{thread, time};

fn main() -> Result<(), std::io::Error> {
    let w = PrintWorker::default();
    // w.println("")?; // prints im working...
    // thread::sleep(time::Duration::from_secs(1));
    // w.stop_working();

    // or

    thread::sleep(time::Duration::from_secs(1));
    w.println("this is work text a");
    thread::sleep(time::Duration::from_secs(1));
    w.println("this is work text b");
    thread::sleep(time::Duration::from_secs(1));
    w.println("this is work text c")
    // also works when w is dropped (finishes work)
}
