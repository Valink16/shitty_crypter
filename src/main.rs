use shitty_crypto;
use std::env::args;
use std::process::exit;
use std::fs::metadata;

fn main() {
    let mut args = args().skip(1); // Skips executable names in args
    
    let original = check_args(args.next());
    let key = check_args(args.next());
    let mode = check_args(args.next());
    let result = check_args(args.next());

    if (mode == "-e" || mode == "-d") && check_file(&original) {
        let key = match key.parse::<isize>() {
            Ok(v) => v,
            Err(_) => {
                println!("Invalid key, enter a number");
                exit_with_usage();
                0
            }
        };

        println!("Crypting {} into {} using mode {} and key {}", original, result, mode, key);
        
        let mode = match mode.as_str() {
            "-e" => shitty_crypto::CryptMode::Encrypt,
            "-d" => shitty_crypto::CryptMode::Decrypt,
            _ => exit(1)
        };

        shitty_crypto::crypt_file(&original, &result, key, mode)
            .expect("Failed to process file");

    } else {
        println!("File does not exist or invalid modes");
        exit_with_usage();
    }
}

fn check_args(v: Option<String>) -> String {
    match v {
        Some(n) => n,
        None => {
            exit_with_usage();
            String::new()
        }
    }
}

fn exit_with_usage() {
    println!("Usage: shitty_crypter <original filename> <key(number)> <mode('-e' or '-d'> <result filename>");
    exit(0);
}

fn check_file(path: &str) -> bool {
    match metadata(path) {
        Ok(md) => md.is_file(),
        Err(_) => false
    }
}