use crate::{cmd::Base64Format, get_reader};
use anyhow::Result;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine,
};
use std::io::Read;

pub fn process_encode(input: &str, format: Base64Format) -> Result<String> {
    let reader = get_reader(input)?;
    _process_encode(reader, format)
}

fn _process_encode(mut reader: impl Read, format: Base64Format) -> Result<String> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };
    Ok(encoded)
}

pub fn process_decode(input: &str, format: Base64Format) -> Result<String> {
    let reader = get_reader(input)?;
    _process_decode(reader, format)
}

fn _process_decode(mut reader: impl Read, format: Base64Format) -> Result<String> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    // avoid accidental newlines
    let buf = buf.trim();

    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };

    let decoded = String::from_utf8(decoded)?;
    Ok(decoded)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_process_base64() {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        let encoded = _process_encode(Cursor::new(input), format).unwrap();

        let output = _process_decode(Cursor::new(encoded), format).unwrap();
        assert_eq!(input, output);
    }
}
