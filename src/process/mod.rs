mod base64;
mod csv_convert;
mod gen_pass;
mod http;
mod text;
mod time;

use std::io::Read;

use anyhow::Result;

pub use base64::{process_decode, process_encode};
pub use csv_convert::process_csv;
pub use gen_pass::process_genpass;
pub use http::process_http_serve;
pub use text::*;
pub use time::process_time;

pub enum Reader {
    Stdin(std::io::Stdin),
    File(std::fs::File),
}

impl Read for Reader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            Self::Stdin(stdin) => stdin.read(buf),
            Self::File(file) => file.read(buf),
        }
    }
}

pub fn get_reader(input: &str) -> Result<Reader> {
    let reader = if input == "-" {
        Reader::Stdin(std::io::stdin())
    } else {
        Reader::File(std::fs::File::open(input)?)
    };
    Ok(reader)
}

pub fn get_content(input: &str) -> anyhow::Result<Vec<u8>> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}
