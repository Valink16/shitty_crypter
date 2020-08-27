use std::fs::{File, rename, remove_file};
use std::io::{BufReader, BufWriter};
use std::io::{Read, Write};

static MAX: isize = std::u8::MAX as isize;
static MIN: isize = std::u8::MIN as isize;

pub enum CryptMode {
    Encrypt,
    Decrypt
}

pub fn crypt_file(file_name: &str, final_name: &str, key: isize, mode: CryptMode) -> Result<(), std::io::Error> {
    let key = get_key(key, mode);
    
    let source = BufReader::new(match File::open(file_name) {
        Ok(f) => f,
        Err(e) => return Result::Err(e)
    });

    let mut result = BufWriter::new(match File::create("tmp") {
        Ok(f) => f,
        Err(e) => return Result::Err(e)
    });

    for c in source.bytes() {
        let v = c.unwrap();
        let new_v = add_wraparound_u8(v, key);
        if let Err(e) = result.write(&[new_v]) {
            return Result::Err(e);
        }
    }

    if final_name == file_name {
        remove_file(file_name).unwrap();
    }
    rename("tmp", final_name).unwrap();
    Ok(())
}

fn get_key(k: isize, mode: CryptMode) -> isize {
    match mode {
        CryptMode::Encrypt => k,
        CryptMode::Decrypt => -k
    }
}

fn add_wraparound_u8(a: u8, b: isize) -> u8 {
    let r = a as isize + b;
    if r > MAX {
        return (r % MAX - 1) as u8;
    } else if r < MIN {
        return (MAX - (r.abs() % MAX) - 1) as u8;
    }
    r as u8
}