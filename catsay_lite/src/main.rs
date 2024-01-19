// use colored::Colorize;
use std::env;
use std::io::{self, Read, Write};
use std::path::PathBuf;

const HELP_MESSAGE: &str = "
Usage: cat [OPTIONS] [MESSAGE]
Display a cat with a message.

Options:
    -d, --dead       Make the cat appear dead.
    -f, --file FILE  Load the cat picture from the specified file.
    -i, --stdin      Read the message from STDIN instead of the argument.
    -h, --help       Print this help message and exit.

Args:
    MESSAGE          What does the cat say? Default is 'Meow!'.
";

struct Options {
    message: String,
    dead: bool,
    catfile: Option<PathBuf>,
    stdin: bool,
}

impl Options {
    fn new() -> Result<Self, &'static str> {
        let args: Vec<String> = env::args().collect();
        let mut message = "Meow!".to_string();
        let mut dead = false;
        let mut catfile = None;
        let mut stdin = false;

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--dead" | "-d" => {
                    dead = true;
                }
                "--file" | "-f" => {
                    i += 1;
                    if i >= args.len() {
                        return Err("Option '--file' requires an argument");
                    }
                    catfile = Some(PathBuf::from(&args[i]));
                }
                "--stdin" | "-i" => {
                    stdin = true;
                }
                "--help" | "-h" => {
                    println!("{}", HELP_MESSAGE);
                    std::process::exit(0);
                }
                _ => {
                    if args[i].starts_with('-') {
                        return Err("Unknown option");
                    }
                    if message != "Meow!" {
                        return Err("Only one message argument is allowed");
                    }
                    message = args[i].clone();
                }
            }
            i += 1;
        }

        Ok(Self {
            message,
            dead,
            catfile,
            stdin,
        })
    }
}

fn main() -> io::Result<()> {
    let options = match Options::new() {
        Ok(o) => o,
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("{}", HELP_MESSAGE);
            return Ok(());
        }
    };

    let message = if options.stdin {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    } else {
        options.message
    };

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog.")
    }

    let eye = if options.dead { "x" } else { "o" };
    // let eye = format!("{}", eye.red().bold());

    let cat_template = match &options.catfile {
        Some(path) => std::fs::read_to_string(path)
            .unwrap_or_else(|_| panic!("Could not read file {:?}", path)),
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
        message //.bright_yellow().underline().on_purple()
    )?;
    writeln!(handle, "{}", &cat_picture)?;
    Ok(())
}
