use std::env::args;
use std::fs::File;
use std::io::{stdin, stdout, Read, Result, Write};

fn main() -> Result<()> {
    let args = args();
    let mut buffer = String::new();
    let mut input = stdin().lock();
    input.read_to_string(&mut buffer)?;
    for (i, arg) in args.enumerate() {
        if i != 0 {
            let mut outfile = File::create(arg)?;
            outfile.write_all(buffer.as_bytes())?;
        }
    }
    stdout().lock().write_all(b"Hello, World!\n")?;
    Ok(())
}
