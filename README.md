# Poindexter

poindexter is a rust library for printing output while also signaling intended work wait times

## Installation

Add this to your cargo.toml

`poindexter = "0.0.1"`

## Usage

```rust
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

```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
[MIT](https://choosealicense.com/licenses/mit/)

