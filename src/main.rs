#[allow(dead_code)]
#[allow(unused_variables)]
use termion::color;

use std::error::Error;
use std::result;
use std::io;
use std::io::{BufRead, Write};
use std::fs;

// use std::{thread, time};

type Result<T> = result::Result<T, Box<Error>>;

#[derive(Debug, PartialEq)]
enum GzipComponent {
    Single(usize),
    Span(usize, usize),
}

struct Gzipped {
    raw: Vec<u8>,
    components: Vec<GzipComponent>,
}

impl Gzipped {
    pub fn new(bytes: &[u8]) -> Self {
        // this implements a very primitive but hopefully easy-to-understand
        // compression alg.
        //
        // raw data is stored in a vector of bytes.  when saving a sequence of
        // bytes that have already-been seen, it will save a Span(start, end)
        // that denotes the range of raw bytes to look up.  otherwise it will
        // save a not-yet-seen byte or sequence of bytes at the end of the raw
        // vector and a Single(ix) / Span(start, end) in its place.

        // accumulating vector of raw bytes
        let mut raw = Vec::new();
        // accumulating vector of components that reference the raw vec
        let mut components = Vec::new();
        let mut start_ix = 0;
        let mut end_ix = 0;
        let mut read_position = 0;
        let mut read_raw_position = 0;
        let mut in_match = false;
        while (read_position) < bytes.len() {
            let current_byte_to_find = bytes[read_position];
            while read_raw_position < raw.len() {
                let current_raw_byte = raw[read_raw_position];
                if current_raw_byte == current_byte_to_find {
                    // found the match!
                    // for this impl we are keeping it simple
                    in_match = true;
                    break;
                } else {
                    read_raw_position += 1;
                    continue;
                }
            }
            if !in_match {
                // not found in raw!
                raw.push(current_byte_to_find);
                components.push(GzipComponent::Single(raw.len() - 1));
            } else {
                // we found it!
                components.push(GzipComponent::Single(read_raw_position));
                in_match = false;
            }
            // start over for next byte to read
            read_raw_position = 0;
            read_position += 1;
        }
        Gzipped {
            raw: raw,
            components: components,
        }
    }

    pub fn get(&self, index: usize) -> Option<Vec<u8>> {
        if let Some(element) = self.components.get(index) {
            match element {
                GzipComponent::Single(b) => Some(vec![self.raw[*b].clone()]),
                GzipComponent::Span(start, end) => {
                    panic!("eh")
                },
            }
        } else {
            None
        }
    }
}


fn main() -> Result<()> {
    /*
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
        let compressed = Gzipped::new(buffs);
        gunzip_print(compressed, &mut handle)?;
    }
    */
    let compressed = Gzipped::new(&[0, 1, 2, 3, 3, 3, 1, 1]);
    use GzipComponent::*;
    assert_eq!(
        vec![
        Single(0),
        Single(1),
        Single(2),
        Single(3),
        Single(3),
        Single(3),
        Single(1),
        Single(1),
        ], compressed.components);
    Ok(())
}

fn gunzip_print<W: Write>(readable: Gzipped, writeable: &mut W) -> Result<()> {
    // let mut ix = 0;
    println!("entering the for loop");
    for c in &readable.components {
        match c {
            GzipComponent::Single(byte) => {
                println!("printing in a for loop");
                //thread::sleep(time::Duration::from_millis(50));
                // if ix % 2 == 0 {
                //     write!(writeable, "{}", color::Bg(color::Yellow))?;
                // }
                //writeable.write()?;
                write!(writeable, "{}", (readable.get(*byte).unwrap()[0] as char))?;
                // if ix % 2 == 0 {
                //     write!(writeable, "{}", color::Bg(color::Reset))?;
                // }
                // ix += 1;
                writeable.flush()?;
            },
            GzipComponent::Span(_start, _end) => {
                // ...
            }
        }
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    fn test_simple_writing() -> Result<()> {
        let s = "asdf";
        let b = s.as_bytes();
        let mut out = vec![];
        gunzip_print(Gzipped::new(&b), &mut out)?;
        assert_eq!(b.to_vec(), out);
        Ok(())
    }

    //#[test]
    fn test_printing() -> Result<()> {
        let s = "abcdabcd";
        let expected = s.as_bytes().to_vec();
        let actual = format!(
            "abcd{}abcd{}", color::Bg(color::Yellow), color::Bg(color::Reset)
        ).as_bytes().to_vec();
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_empty() {
        let compressed = Gzipped::new(&[]);
        use GzipComponent::*;
        let expected: Vec<GzipComponent> = Vec::new();
        assert_eq!(expected, compressed.components);
    }

    #[test]
    fn test_no_compression() {
        let compressed = Gzipped::new(&[0, 1, 2]);
        use GzipComponent::*;
        assert_eq!(vec![Single(0), Single(1), Single(2)], compressed.components);
    }

    #[test]
    fn test_simple_repettition() {
        let compressed = Gzipped::new(&[0, 1, 2, 3, 3, 3, 1, 1]);
        use GzipComponent::*;
        assert_eq!(compressed.raw.len(), 4);
        assert_eq!(
            vec![
                   Single(0),
                   Single(1),
                   Single(2),
                   Single(3),
                   Single(3),
                   Single(3),
                   Single(1),
                   Single(1),
            ], compressed.components);
    }

    //#[test]
    fn test_some_compression() {
        let compressed = Gzipped::new(&[0, 1, 2, 3, 0, 1, 2, 3]);
        use GzipComponent::*;
        assert_eq!(vec![Single(0), Single(1), Single(2), Single(3), Span(0, 3)], compressed.components);
    }

    //#[test]
    fn test_byte_getter() {
        let compressed = Gzipped::new(&[0, 1, 2, 3, 0, 1, 2, 3]);
        assert_eq!(0, compressed.get(0).unwrap()[0]);
    }

}
