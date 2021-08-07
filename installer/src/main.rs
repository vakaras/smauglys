use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::create("C:\\rusty-installer.txt")?;
    file.write_all(b"Hello, world!")?;
    Ok(())
}