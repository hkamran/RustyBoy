use std::path::Path;
use std::fs::{OpenOptions, File};
use std::io::Write;
use std::convert::TryInto;

const FILE_PATH: &str = "cpu.log";

static mut FILE: Option<File> = Option::None;

pub fn log(value: String) {
    println!("{}", value);
    unsafe {
        if FILE.is_none() {
            FILE = Option::Some(OpenOptions::new()
                .create(true)
                .write(true)
                .open(FILE_PATH)
                .unwrap());
        }

        FILE.as_mut().unwrap().write_all((value + "\n").as_bytes());

    }
}
