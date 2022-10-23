use std::io::{BufRead, BufReader};

fn main() {
    let input = aoc_2022::input!().map(BufReader::new).unwrap();

    let mut count_decode = 0;
    let mut count_encode = 0;

    for line in input.lines() {
        let line = line.unwrap();

        let decoded = decode(&line);
        let encoded = encode(&line);

        println!("{{{line}}} {{{decoded}}} {{{encoded}}}");

        count_decode += line.len();
        count_decode -= decoded;

        count_encode += encoded - line.len();
    }

    println!("{count_decode}");
    println!("{count_encode}");
}

fn decode(s: &str) -> usize {
    Decode::new(s).count()
}

fn encode(s: &str) -> usize {
    let mut output = 0;
    for byte in s.bytes() {
        match byte {
            b'"' => {
                output += 2;
            }
            b'\\' => {
                output += 2;
            }
            _ => output += 1,
        }
    }

    output + 2
}

struct Decode<'a>(&'a [u8]);

impl<'a> Decode<'a> {
    fn new(s: &'a str) -> Self {
        Self(s.as_bytes())
    }
}

impl<'a> Iterator for Decode<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let (&first, rest) = self.0.split_first()?;
        match first {
            b'"' => {
                self.0 = rest;
                self.next()
            }
            b'\\' => match rest.split_first() {
                Some((&c @ (b'"' | b'\\'), rest)) => {
                    self.0 = rest;
                    Some(c)
                }
                Some((b'x', rest)) => {
                    if let [a, b, rest @ ..] = rest {
                        let a = convert_ascii_to_num(*a)?;
                        let b = convert_ascii_to_num(*b)?;
                        self.0 = rest;
                        Some(a * 16 + b)
                    } else {
                        None
                    }
                }
                _ => todo!(),
            },
            c => {
                self.0 = rest;
                Some(c)
            }
        }
    }
}

fn convert_ascii_to_num(byte: u8) -> Option<u8> {
    match byte {
        b @ b'0'..=b'9' => Some(b - b'0'),
        b @ b'a'..=b'f' => Some(b - b'a' + 10),
        _ => None,
    }
}
