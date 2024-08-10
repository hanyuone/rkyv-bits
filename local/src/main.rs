use std::{fs, io};

use local::Foo;

fn main() -> io::Result<()> {
    let foo = Foo { content: 128 };
    let bytes = rkyv::to_bytes::<_, 1_024>(&foo).unwrap();

    let target_path = "wasm/public/archived";
    fs::write(target_path, bytes)
}
