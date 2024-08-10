use std::{fs, io};

use local::Foo;
use rkyv::rancor::Error;

fn main() -> io::Result<()> {
    // Breaks for any integer >= 128
    let foo = Foo { content: 128 };
    let bytes = rkyv::to_bytes::<Error>(&foo).unwrap();

    let target_path = "wasm/public/archived";
    fs::write(target_path, bytes)
}
