use clap::Parser;
use rand::Rng;

/// A simple CLI for generating secure passwords written in Rust
#[derive(Parser)]
#[clap(version, about, long_about= None)]
struct Cli {
    /// Length of password
    #[clap(short, long, default_value_t = 30)]
    length: u8,

    /// Include symbols
    #[clap(short, long)]
    symbols: bool,
}

fn main() {
    let cli = Cli::parse();

    let mut charset: Vec<u8> = Vec::new();

    let mut alphanumeric: Vec<u8> =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".to_vec();
    let mut symbols: Vec<u8> = b")(*&^%$#@!~".to_vec();

    charset.append(&mut alphanumeric);
    if cli.symbols {
        charset.append(&mut symbols);
    }

    let password_length: u8 = cli.length;

    let mut rng = rand::thread_rng();

    let password: String = (0..password_length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect();

    println!("{}", password);
}
