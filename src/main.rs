use std::io::{self, Read, Write};
use clap::{Arg,App};
use vte::{Params, Parser, Perform};
use unicode_width::UnicodeWidthChar;

struct Cut {
    len: usize,
    pos: usize
}

impl Cut {
    fn new(len: usize) -> Self {
        Self { len, pos: 0 }
    }
}

impl Perform for Cut {
    fn print(&mut self, c: char) {
        self.pos += UnicodeWidthChar::width(c).unwrap_or(0);
        if self.pos <= self.len {
            print!("{}", c);
        }
    }

    fn execute(&mut self, byte: u8) {
        match byte {
            0x0a | 0x0c | 0x0d => self.pos = 0,
            _ => ()
        };
        if byte == 0x09 {
            let tab = 8 * ((self.pos + 8) / 8);
            while self.pos < tab && self.pos < self.len {
                print!(" ");
                self.pos += 1;
            }
        } else {
            match io::stdout().write_all(&[byte]) {
                Ok(()) => (),
                Err(e) => panic!(e)
            }
        }
    }

    fn hook(&mut self, _params: &Params, _intermediates: &[u8], _ignore: bool, _c: char) {
    }

    fn put(&mut self, _byte: u8) {
    }

    fn unhook(&mut self) {
    }

    fn osc_dispatch(&mut self, _params: &[&[u8]], _bell_terminated: bool) {
    }

    fn csi_dispatch(&mut self, params: &Params, intermediates: &[u8], _ignore: bool, c: char) {
        if c == 'm' {
            let mut v: Vec<u8> = Vec::new();
            v.push(0x1b);
            v.push(0x5b);
            for i in intermediates {
                v.push(*i);
            }
            if !params.is_empty() {
                let mut iter = params.iter();
                write!(&mut v, "{}", iter.nth(0).unwrap()[0]).unwrap();
                for p in iter {
                    write!(&mut v, ";{}", p[0]).unwrap();
                }
            }
            v.push(0x6d);
            match io::stdout().write_all(v.as_slice()) {
                Ok(()) => (),
                Err(e) => panic!(e)
            };
        }

    }

    fn esc_dispatch(&mut self, intermediates: &[u8], ignore: bool, byte: u8) {
        if !ignore {
            let mut v: Vec<u8> = Vec::new();
            v.push(0x1b);
            for i in intermediates {
                v.push(*i);
            }
            v.push(byte);
            match io::stdout().write_all(v.as_slice()) {
                Ok(()) => (),
                Err(e) => panic!(e)
            };
        }
    }
}

fn run(len: usize) {
    let input = io::stdin();
    let mut handle = input.lock();

    let mut statemachine = Parser::new();
    let mut performer = Cut::new(len);

    let mut buf = [0; 2048];

    loop {
        match handle.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                for byte in &buf[..n] {
                    statemachine.advance(&mut performer, *byte);
                }
            },
            Err(err) => panic!(err)
        }
    }
}

fn main() {
    let args = App::new("color-cut")
        .version("0.1.0")
        .author("Florian Hars")
        .about("Cut potentially colored output of a program to a visible number of columns.")
        .arg(Arg::with_name("LEN")
             .help("The desired line length of the output.")
             .required(true)
             .index(1))
        .get_matches();

    let arg = args.value_of("LEN").unwrap();
    match arg.parse::<usize>() {
        Ok(len) => run(len),
        Err(_) => panic!("line length must be a positive number")
    }

}
