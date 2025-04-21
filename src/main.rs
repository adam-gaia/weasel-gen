use anyhow::{bail, Result};
use clap::Parser;
use rand::Rng;
use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
};

const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const NUMBERS: &str = "0123456789";

#[derive(Parser)]
struct Cli {
    /// String to generate
    #[clap(default_value = "Hello, World!")]
    string: Vec<String>,

    /// Weather or not to show stats at the end
    #[clap(short, long)]
    stats: bool,

    /// Print delay between iterations (milliseconds)
    #[clap(short, long, default_value = "50")]
    delay: u64,

    /// Use only a-z, A-Z
    #[clap(short, long)]
    #[arg(group = "charset_group")]
    alpha: bool,

    /// Use a-z, A-Z, 0-9
    #[clap(long)]
    #[arg(group = "charset_group")]
    alphanum: bool,

    /// Supply a custom character set
    #[clap(short, long)]
    #[arg(group = "charset_group")]
    charset: Option<String>,

    /// TODO
    #[clap(short, long, default_value = "3")]
    factor: usize,
}

/// Get a random ascii char (restricted to printable characters)
fn rand_char<T: Rng>(rng: &mut T, charset: &[u8]) -> char {
    let i = rng.random_range(0..charset.len());
    charset[i] as char
}

/// Create a Vec of size N with random chars
fn rand_vec<T: Rng>(rng: &mut T, charset: &[u8], n: usize) -> Vec<char> {
    let mut data = Vec::with_capacity(n);
    for _ in 0..n {
        data.push(rand_char(rng, charset));
    }
    data
}

/// Mutate input data slice randomly one iteration.
/// Returns true if more generations are needed, false to signal that we've hit the target.
/// A "cheat factor" may be set to increase the chances of generating the target
fn mutate<T: Rng>(
    rng: &mut T,
    charset: &[u8],
    n: usize,
    target: &[char],
    data: &mut [char],
    cheat_factor: usize,
) -> bool {
    let mut not_done = false;
    for _ in 0..cheat_factor {
        for i in 0..n {
            if data[i] != target[i] {
                data[i] = rand_char(rng, charset);

                // If we didn't guess our rarget, we will need to do another loop
                if data[i] != target[i] {
                    not_done = true;
                }
            }
        }
    }
    not_done
}

/// Print data (using a pre-allocated buffer), overwriting the current line
fn print(buf: &mut String, data: &[char]) -> Result<()> {
    buf.clear();
    for &c in data {
        buf.push(c);
    }
    print!("\r{}", buf);
    stdout().flush()?;
    Ok(())
}

/// Make sure target string only contains characters found in the charset
fn valid(s: &str, charset: &[u8]) -> Result<()> {
    for &b in s.as_bytes() {
        if !charset.contains(&b) {
            let fmt_charset: Vec<char> = charset.iter().map(|x| *x as char).collect();
            // TODO: would be really cool to use annotate_snippets to display this error message
            bail!(
                "Target character '{}' not in charset {fmt_charset:?}",
                b as char
            );
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let factor = args.factor;

    if factor == 0 {
        bail!("Factor must be greater than 0");
    }

    let stats = args.stats;
    let delay = args.delay;

    let charset = if let Some(charset) = args.charset {
        charset
    } else if args.alpha {
        ALPHABET.to_owned()
    } else if args.alphanum {
        ALPHABET.to_owned() + NUMBERS
    } else {
        (0x20u8..=0x7E).map(|x| x as char).collect()
    };
    let charset = charset.as_bytes();

    let target = args.string.join(" ");
    valid(&target, &charset)?;
    let target: Vec<char> = target.chars().collect();

    let mut rng = rand::rng();

    let n = target.len();
    let mut data = rand_vec(&mut rng, charset, n);

    let delay = Duration::from_millis(delay);

    let mut buf = String::with_capacity(n);

    print(&mut buf, &data)?;

    let mut iterations = 0;
    while mutate(&mut rng, charset, n, &target, &mut data, factor) {
        iterations += 1;
        thread::sleep(delay);
        print(&mut buf, &data)?;
    }
    print(&mut buf, &data)?;
    println!();

    if stats {
        let note = if factor > 1 {
            format!(" (with a {factor}x generation factor)")
        } else {
            String::new()
        };
        println!("Generated in {iterations} iterations{note}");
    }

    Ok(())
}
