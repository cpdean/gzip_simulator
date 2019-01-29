#[allow(dead_code)]
#[allow(unused_variables)]
use termion::color;

use std::error::Error;
use std::fs;
use std::io;
use std::io::{BufRead, Write};
use std::result;

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
        if bytes.len() == 0 {
            return Gzipped {
                raw: raw,
                components: components,
            }
        }
        let mut current_byte_to_find = bytes[read_position];
        let mut _view_raw = raw.as_slice();
        let mut _view_comp = components.as_slice();
        while read_position < bytes.len() {
            current_byte_to_find = bytes[read_position];
            while read_raw_position < dbg!(raw.len()) {
                _view_raw = raw.as_slice();
                _view_comp = components.as_slice();
                let current_raw_byte = raw[dbg!(read_raw_position)];
                if current_raw_byte == current_byte_to_find {
                    // found the match!
                    if !in_match {
                        dbg!("oh boy a match");
                        in_match = true;
                        // record start of this match segment
                        start_ix = read_raw_position;
                        // start end_ix at the same so if we only match this of width 1
                        // we know to write a Single(usize) out
                        end_ix = read_raw_position;
                    } else {
                        // already in a match, so record the successful end_ix of this span
                        end_ix = read_raw_position;
                    }
                    // move both pointers to continue scannning
                    read_raw_position += 1;
                    read_position += 1;
                    if read_position < bytes.len() {
                        current_byte_to_find = bytes[read_position];
                    } else {
                        break;
                    }
                } else {
                    if !in_match {
                        read_raw_position += 1;
                        continue;
                    } else {
                        // you were in a match!
                        // commit what you have as a gzip component, and then rescan from the
                        // beginning on the current byte
                        if start_ix == end_ix {
                            // since the indexes match, we only found a single character.
                            // TODO: start over and scan for better matches for the starter byte on
                            // input buffer
                            // the above todo is harder than just giving up immediately.
                            components.push(GzipComponent::Single(start_ix));
                            in_match = false;
                            dbg!("could not find a match for this guy");
                            dbg!(current_byte_to_find);
                            dbg!("position is");
                            dbg!(read_position);
                            dbg!("resetting the raw pos because i got a non match after getting a match");
                            read_raw_position = 0;
                        } else {
                            // the indexes do not match, so we have a span to commit
                            components.push(GzipComponent::Span(start_ix, end_ix));
                            in_match = false;
                            read_raw_position = 0;
                        }
                    }
                }
            }
            // you've exhausted the current raw buffer, so it must be extended if we have not found
            // a match
            if !in_match {
                // not found in raw!
                raw.push(current_byte_to_find);
                components.push(GzipComponent::Single(raw.len() - 1));
            } else {
                // you are in a match, but because we got to the end we need to commit the on-going
                // span AND push on the current byte
                if start_ix == end_ix {
                    // we only matched a single byte from the raw buffer
                    components.push(GzipComponent::Single(start_ix));
                    read_raw_position = 0;
                    in_match = false;
                } else {
                    components.push(GzipComponent::Span(start_ix, end_ix));
                    read_raw_position = 0;
                    in_match = false;
                }
                // did we run off the end or finish perfectly?
                if read_position + 1 == bytes.len() {
                    // perfect finish
                    break;
                } else {
                    // we have more bytes to read, start by pushing the current byte into raw and
                    // recording it
                    // BUG: if i'm pushing into raw but i haven't actually scanned the whole raw for the current byte,
                    // I should not be pushing into raw.
                    //raw.push(current_byte_to_find);
                    //components.push(GzipComponent::Single(raw.len() - 1));
                    continue;
                }
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
                GzipComponent::Span(start, end) => panic!("eh"),
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
    //assert_eq!(compressed.raw.len(), 4);
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
        ],
        compressed.components
    );
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
            }
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
            "abcd{}abcd{}",
            color::Bg(color::Yellow),
            color::Bg(color::Reset)
        )
        .as_bytes()
        .to_vec();
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
        //assert_eq!(compressed.raw.len(), 4);
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
            ],
            compressed.components
        );
    }

    #[test]
    fn test_simple_repettition2() {
        let compressed = Gzipped::new(&[0, 1, 2, 3, 4, 3, 1, 1]);
        use GzipComponent::*;
        assert_eq!(
            vec![
                Single(0),
                Single(1),
                Single(2),
                Single(3),
                Single(4),
                Single(3),
                Single(1),
                Single(1),
            ],
            compressed.components
        );
    }

    #[test]
    fn test_some_compression() {
        let compressed = Gzipped::new(&[0, 1, 2, 3, 0, 1, 2, 3]);
        use GzipComponent::*;
        assert_eq!(
            vec![Single(0), Single(1), Single(2), Single(3), Span(0, 3)],
            compressed.components
        );
    }

    #[test]
    fn test_bigger_compression() {
        let compressed = Gzipped::new(&[0, 1, 2, 3, 4, 0, 1, 2, 3, 4]);
        use GzipComponent::*;
        assert_eq!(
            vec![Single(0), Single(1), Single(2), Single(3), Single(4), Span(0, 4)],
            compressed.components
        );
    }

    #[test]
    fn test_bigger_compression_with_repeat() {
        let compressed = Gzipped::new(&[0, 1, 2, 3, 4, 0, 1, 2, 3, 4, 0, 1, 2, 3, 4]);
        use GzipComponent::*;
        assert_eq!(
            vec![Single(0), Single(1), Single(2), Single(3), Single(4), Span(0, 4), Span(0, 4)],
            compressed.components
        );
    }

    #[test]
    fn test_bigger_compression_with_repeat_and_a_break() {
        let compressed = Gzipped::new(&[0, 1, 2, 3, 4, 0, 1, 2, 3, 4, 5, 0, 1, 2, 3, 4]);
        use GzipComponent::*;
        assert_eq!(
            vec![Single(0), Single(1), Single(2), Single(3), Single(4), Span(0, 4), Single(5), Span(0, 4)],
            compressed.components
        );
    }

    //#[test]
    fn test_byte_getter() {
        let compressed = Gzipped::new(&[0, 1, 2, 3, 0, 1, 2, 3]);
        assert_eq!(0, compressed.get(0).unwrap()[0]);
    }

}
