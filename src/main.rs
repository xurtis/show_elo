//! ELO Serial touch interface driver.
//!
//! The messages coming from the touch screen come in 5-byte packets.
//!
//! Byte 0 is a set of flags.
//! Bytes 1 and 2 are the X axis absolute position.
//! Bytes 3 and 4 are the X axis absolute position.

use serial::unix::TTYPort;
use std::path::Path;
use std::io::Read;
use std::thread::sleep;
use std::time::Duration;

const ELO_SERIAL: &'static str = "/dev/ttyS4";

const ELO_FLAGS_ON:    u16 = 0b10001000;
const ELO_FLAGS_OFF:   u16 = 0b00110111;
const ELO_FLAGS_PRESS: u16 = 0b01000000;

macro_rules! check_flags {
    ($value:expr, $flags:expr) => {
        ($value & $flags) == $flags
    }
}

fn main() {
    let serial = TTYPort::open(Path::new(ELO_SERIAL)).expect("Opening serial device");
    let mut bytes = serial.bytes()
        .filter(|b| b.is_ok())
        .map(|b| b.ok().unwrap() as u16);

    loop {
        let flags = read_flags(&mut bytes);
        let mut x = 0u16;
        x |= bytes.next().unwrap() << 8;
        x |= bytes.next().unwrap();
        let mut y = 0u16;
        y |= bytes.next().unwrap() << 8;
        y |= bytes.next().unwrap();

        println!("[{:7?}]: {:>5}x {:>5}y", flags, x, y);
    }
}

#[derive(Debug)]
enum Direction {
    Press,
    Release,
}
use self::Direction::*;

fn read_flags(bytes: &mut impl Iterator<Item = u16>) -> Direction {
    loop {
        let byte = bytes.next().unwrap();

        if !check_flags!(byte, ELO_FLAGS_ON) {
            continue;
        }
        if !check_flags!(!byte, ELO_FLAGS_OFF) {
            continue;
        }
        if check_flags!(byte, ELO_FLAGS_PRESS) {
            break Press;
        } else {
            break Release;
        }
    }
}
