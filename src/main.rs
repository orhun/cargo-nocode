use std::env;
use std::error::Error;
use std::fs::File;
use std::process;

const USAGE_TEXT: &str = "
Usage: cargo-nocode <command>

Possible commands:
- init    initialize a nocode applicaton
- build   build the nocode application
- run     run the nocode application
- deploy  deploy the nocode application
";

fn usage() {
    println!("No command given.");
    println!("{USAGE_TEXT}");
    process::exit(1)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        usage()
    }

    let command = &args[1];
    match command.as_ref() {
        "init" => {
            File::create("nocode.rs")?;
            println!("Created nocode.rs! Now start by writing not any code.");
        }
        "build" | "run" | "deploy" => {}
        _ => usage(),
    }

    Ok(())
}
