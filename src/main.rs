use std::io::Read;

use anyhow::Context;
use clap::Parser;

/// Prints the `key=value` pairs of a text as JSON.
#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// The text to read; standard input when left out.
    text: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let text = match args.text {
        Some(text) => text,
        None => {
            let mut text = String::new();
            std::io::stdin()
                .read_to_string(&mut text)
                .context("reading standard input")?;
            text
        }
    };
    println!("{}", bake_demo::to_json(&bake_demo::pairs(&text))?);
    Ok(())
}
