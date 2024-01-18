use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use std::io::{self, Read, Write};

#[derive(Parser)]
struct Options {
    /// What does the cat say?
    #[clap(default_value = "Meow!")]
    message: String,

    /// Make the cat appear dead
    #[clap(short = 'd', long = "dead")]
    dead: bool,

    /// Load the cat picture from the specified file
    #[clap(short = 'f', long = "file")]
    catfile: Option<std::path::PathBuf>,

    /// Read the message from STDIN instead of the argument
    #[clap(short = 'i', long = "stdin")]
    stdin: bool,
}

fn main() -> Result<()> {
    let options = Options::parse();
    let mut message = String::new();
    if options.stdin {
        io::stdin().read_to_string(&mut message)?;
    } else {
        message = options.message;
    };

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog.")
    }

    let eye = if options.dead { "x" } else { "o" };
    let eye = format!("{}", eye.red().bold());

    let cat_template = match &options.catfile {
        Some(path) => std::fs::read_to_string(path)
            .with_context(|| format!("Could not read file {:?}", path))?,
        None => String::from(
            "
 \\
  \\
    /\\_/\\
   ( {eye} {eye} )
   =( I )=",
        ),
    };

    let cat_picture = cat_template.replace("{eye}", &eye);
    writeln!(
        handle,
        "{}",
        message.bright_yellow().underline().on_purple()
    )?;
    writeln!(handle, "{}", &cat_picture)?;
    Ok(())
}
