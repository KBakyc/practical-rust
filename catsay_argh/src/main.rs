use anyhow::{Context, Result};
use argh::FromArgs;
use colored::Colorize;
use std::io::{self, Read, Write};

#[derive(FromArgs, Debug)]
/// Prints a cat saying something
struct Options {
    /// what does the cat say?
    // #[argh(positional, default = "Meow!")]
    #[argh(positional, default = "String::from(\"Meow!\")")]
    message: String,

    /// read the message from STDIN instead of the argument
    #[argh(switch, short = 'i')]
    stdin: bool,

    /// make the cat appear dead
    #[argh(switch, short = 'd')]
    dead: bool,

    /// load the cat picture from the specified file
    #[argh(option, short = 'f', long = "file")]
    catfile: Option<std::path::PathBuf>,
}

fn main() -> Result<()> {
    let options: Options = argh::from_env();
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
