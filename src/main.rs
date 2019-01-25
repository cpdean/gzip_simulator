use termion::{color, style};

use std::error::Error;
use std::result;
use std::io;
use std::io::{BufRead, Write};
use std::env;
use std::fs;

type Result<T> = result::Result<T, Box<Error>>;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        println!("USAGE");
        println!("{} FILE...", args[0]);
    } else if args.len() == 2 {
        let file_name = &args[1];
        let f = fs::File::open(file_name)?;
        let mut reader = io::BufReader::new(f);
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        let buffs = reader.fill_buf()?;
        gunzip_print(buffs, &mut handle)?;
    }
    Ok(())
}

fn gunzip_print<W: Write>(readable: &[u8], writeable: &mut W) -> Result<()> {
    for c in readable {
        writeable.write(&[*c])?;
    }
    Ok(())
}
