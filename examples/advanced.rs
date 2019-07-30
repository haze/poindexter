use poindexter::PrintWorker;
use std::{thread, time};

fn main() -> Result<(), std::io::Error> {
    let w = PrintWorker::default();
    w.println("Building rockets")?;
    thread::sleep(time::Duration::from_secs(1));
    w.println("Launching rockets")?;
    thread::sleep(time::Duration::from_secs(1));
    w.println("Terraforming Mars")?;
    thread::sleep(time::Duration::from_secs(1));
    w.println("Colonizing")
}
