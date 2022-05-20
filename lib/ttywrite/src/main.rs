mod parsers;

use clap::Parser;
use serial::core::{BaudRate, CharSize, FlowControl, StopBits};
use serial::{self, SerialPort};
use std::fs::File;
use std::io::{self, BufReader};
use std::path::PathBuf;
use std::time::Duration;
use xmodem::{Progress, Xmodem};

use parsers::{parse_baud_rate, parse_flow_control, parse_stop_bits, parse_width};

#[derive(Parser, Debug)]
#[clap(about = "Write to TTY using the XMODEM protocol by default.")]
struct Opt {
    #[clap(
        short = 'i',
        help = "Input file (defaults to stdin if not set)",
        parse(from_os_str)
    )]
    input: Option<PathBuf>,

    #[clap(
        short = 'b',
        long = "baud",
        parse(try_from_str = parse_baud_rate),
        help = "Set baud rate",
        default_value = "115200"
    )]
    baud_rate: BaudRate,

    #[clap(
        short = 't',
        long = "timeout",
        parse(try_from_str),
        help = "Set timeout in seconds",
        default_value = "10"
    )]
    timeout: u64,

    #[clap(
        short = 'w',
        long = "width",
        parse(try_from_str = parse_width),
        help = "Set data character width in bits",
        default_value = "8"
    )]
    char_width: CharSize,

    #[clap(help = "Path to TTY device", parse(from_os_str))]
    tty_path: PathBuf,

    #[clap(
        short = 'f',
        long = "flow-control",
        parse(try_from_str = parse_flow_control),
        help = "Enable flow control ('hardware' or 'software')",
        default_value = "none"
    )]
    flow_control: FlowControl,

    #[clap(
        short = 's',
        long = "stop-bits",
        parse(try_from_str = parse_stop_bits),
        help = "Set number of stop bits",
        default_value = "1"
    )]
    stop_bits: StopBits,

    #[clap(short = 'r', long = "raw", help = "Disable XMODEM")]
    raw: bool,
}

fn main() {
    let opt = Opt::parse();
    let mut port = serial::open(&opt.tty_path).expect("path points to invalid TTY");

    // configure port
    let settings = serial::PortSettings {
        baud_rate: opt.baud_rate,
        char_size: opt.char_width,
        parity: serial::core::ParityNone,
        stop_bits: opt.stop_bits,
        flow_control: opt.flow_control,
    };
    port.configure(&settings).expect("port config failed");
    port.set_timeout(Duration::new(opt.timeout, 0))
        .expect("failed to set timeout");

    // get input
    let mut input: Box<dyn io::Read> = match opt.input {
        Some(file) => Box::new(BufReader::new(File::open(file).unwrap())),
        None => Box::new(BufReader::new(io::stdin())),
    };

    // transmit
    if opt.raw {
        io::copy(&mut input, &mut port);
    } else {
        Xmodem::transmit_with_progress(input, port, progress_fn).unwrap();
    }
}

fn progress_fn(progress: Progress) {
    println!("Progress: {:?}", progress);
}
