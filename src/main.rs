use termion::color;

use std::error::Error;
use std::result;
use std::io;
use std::io::{BufRead, Write};
use std::fs;

use std::{thread, time};

type Result<T> = result::Result<T, Box<Error>>;

enum GzipComponent {
    Byte(u8),
    Span(usize, usize),
}

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
    let mut ix = 0;
    for c in readable {
        //thread::sleep(time::Duration::from_millis(50));
        // if ix % 2 == 0 {
        //     write!(writeable, "{}", color::Bg(color::Yellow))?;
        // }
        writeable.write(&[*c])?;
        // if ix % 2 == 0 {
        //     write!(writeable, "{}", color::Bg(color::Reset))?;
        // }
        ix += 1;
        writeable.flush()?;
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it_works() {
        let s = "asdf";
        let b = s.as_bytes();
        let mut out = vec![];
        gunzip_print(&b, &mut out);
        assert_eq!(b.to_vec(), out);
    }

}
