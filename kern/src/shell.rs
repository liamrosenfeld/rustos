// use shim::io;
// use shim::path::{Path, PathBuf};

use core::fmt::Debug;
use core::iter::Iterator;
use core::panic;
use core::prelude::rust_2021::derive;
use core::result::{Result, Result::Err, Result::Ok};
use core::str;
use stack_vec::StackVec;

// use pi::atags::Atags;

// use fat32::traits::FileSystem;
// use fat32::traits::{Dir, Entry};

use crate::console::{kprint, kprintln, CONSOLE};
// use crate::ALLOCATOR;
// use crate::FILESYSTEM;

/// Error type for `Command` parse failures.
#[derive(Debug)]
enum Error {
    Empty,
    TooManyArgs,
}

/// A structure representing a single shell command.
struct Command<'a> {
    args: StackVec<'a, &'a str>,
}

impl<'a> Command<'a> {
    /// Parse a command from a string `s` using `buf` as storage for the
    /// arguments.
    ///
    /// # Errors
    ///
    /// If `s` contains no arguments, returns `Error::Empty`. If there are more
    /// arguments than `buf` can hold, returns `Error::TooManyArgs`.
    fn parse(s: &'a str, buf: &'a mut [&'a str]) -> Result<Command<'a>, Error> {
        let mut args = StackVec::new(buf);
        for arg in s.split(' ').filter(|a| !a.is_empty()) {
            args.push(arg).map_err(|_| Error::TooManyArgs)?;
        }

        if args.is_empty() {
            return Err(Error::Empty);
        }

        Ok(Command { args })
    }

    /// Returns this command's path. This is equivalent to the first argument.
    fn path(&self) -> &str {
        self.args.first().unwrap_or(&"")
    }
}

/// Starts a shell using `prefix` as the prefix for each line. This function
/// returns if the `exit` command is called.
pub fn shell(prefix: &str) -> ! {
    // welcome the user
    kprintln!("WELCOME TO THE SHELL");

    // storage for the input for each line
    let mut line_buf = [0; 512];
    let mut line = StackVec::new(&mut line_buf);

    // keep recieving commands until exit
    loop {
        // start new line
        kprint!("{}", prefix);
        line.truncate(0);

        // grab contents of line
        loop {
            let byte = CONSOLE.lock().read_byte();
            match byte {
                // valid ascii
                32..=126 => {
                    if let Ok(_) = line.push(byte) {
                        CONSOLE.lock().write_byte(byte);
                    }
                }

                // backspace and delete
                8 | 127 => {
                    if line.len() > 0 {
                        CONSOLE.lock().write_byte(8);
                        CONSOLE.lock().write_byte(b' ');
                        CONSOLE.lock().write_byte(8);
                        line.truncate(line.len() - 1);
                    }
                }

                // end on newline
                b'\n' | b'\r' => {
                    kprintln!();
                    break;
                }

                // ring bell for invalid character
                _ => CONSOLE.lock().write_byte(7),
            }
        }

        // get command from that
        let line_str = str::from_utf8(line.as_slice()).unwrap();
        let mut arg_buf = [""; 64];
        match Command::parse(line_str, &mut arg_buf) {
            Ok(cmd) => match cmd.path() {
                "echo" => kprintln!("{}", &line_str[5..]),
                "panic" => panic!("example panic message"),
                _ => kprintln!("unknown command: {}", cmd.path()),
            },
            Err(Error::TooManyArgs) => kprintln!("error: too many arguments"),
            Err(Error::Empty) => {}
        }
    }
}
