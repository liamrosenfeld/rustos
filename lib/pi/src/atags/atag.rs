use core::slice;
use core::str;

use crate::atags::raw;

pub use crate::atags::raw::{Core, Mem};

/// An ATAG.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Atag {
    Core(raw::Core),
    Mem(raw::Mem),
    Cmd(&'static str),
    Unknown(u32),
    None,
}

impl Atag {
    /// Returns `Some` if this is a `Core` ATAG. Otherwise returns `None`.
    pub fn core(self) -> Option<Core> {
        match self {
            Atag::Core(core) => Some(core),
            _ => None,
        }
    }

    /// Returns `Some` if this is a `Mem` ATAG. Otherwise returns `None`.
    pub fn mem(self) -> Option<Mem> {
        match self {
            Atag::Mem(mem) => Some(mem),
            _ => None,
        }
    }

    /// Returns `Some` with the command line string if this is a `Cmd` ATAG.
    /// Otherwise returns `None`.
    pub fn cmd(self) -> Option<&'static str> {
        match self {
            Atag::Cmd(cmd) => Some(cmd),
            _ => None,
        }
    }
}

impl From<&'static raw::Atag> for Atag {
    fn from(atag: &'static raw::Atag) -> Atag {
        unsafe {
            match (atag.tag, &atag.kind) {
                (raw::Atag::CORE, &raw::Kind { core }) => Atag::Core(core),
                (raw::Atag::MEM, &raw::Kind { mem }) => Atag::Mem(mem),
                (raw::Atag::CMDLINE, &raw::Kind { ref cmd }) => {
                    // find the length of the string (go until null terminator)
                    let mut ptr = &cmd.cmd as *const u8;
                    let mut len = 0;
                    while *ptr as char != '\0' {
                        ptr = ptr.add(1);
                        len += 1;
                    }

                    // get string from base and length
                    let slice = slice::from_raw_parts(&cmd.cmd as *const u8, len);
                    Atag::Cmd(str::from_utf8(slice).unwrap())
                }
                (raw::Atag::NONE, _) => Atag::None,
                (id, _) => Atag::Unknown(id),
            }
        }
    }
}
