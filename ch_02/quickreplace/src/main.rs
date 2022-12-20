use text_colorizer::*;

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    ouput: String,
}

// -----------------------------------------------------------------------------

fn print_usage() {
    eprintln!("{} - change occurrences of one string into another", "quickreplace".green());
    eprintln!("Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>");
}


// -----------------------------------------------------------------------------
use std::env;

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        print_usage();
        eprintln!("{} wrong number of arguments: expeccted 4, got {}.", 
            "Error:".red().bold(), args.len());
        std::process::exit(-1);
    }

    Arguments { 
        target: args[0].clone(), 
        replacement: args[1].clone(), 
        filename: args[2].clone(), 
        ouput: args[3].clone(),
    }
}


// -----------------------------------------------------------------------------

use regex::Regex;
fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let re = Regex::new(target)?;
    Ok(re.replace_all(text, replacement).to_string())
}

// -----------------------------------------------------------------------------


fn main() {
    let args = parse_args();
    println!("{:?}", args);

    use std::fs;

    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to read from file '{}': {:?}", "Error:".red().bold(), args.filename, e);
            std::process::exit(-1);
        }
    };

    let replace_data = match replace(&args.target, &args.replacement, &data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to replace: {:?}", "Error:".red().bold(), e);
            std::process::exit(-1);
        }
    };

    match fs::write(&args.ouput, &replace_data) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{} failed to write to file '{}': {:?}", "Error:".red().bold(), args.ouput, e);
            std::process::exit(-1);
        }
    };
}
