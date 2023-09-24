use clap::Parser;
use std::fs;
use std::process::Command;
use anyhow::Result;
use std::path::Path;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value = "./")]
    local_path: String,

    #[arg(long, default_value = "backblaze-b2")]
    b2_exec: String,

    #[arg(long)]
    b2_path: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let path = args.local_path;
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                println!("{:?}", entry.file_name());
            }
        }
    }

    let output = Command::new(args.b2_exec).arg("ls").arg("--recursive").arg(args.b2_path).output()?;
    let list = String::from_utf8_lossy(&output.stdout);
    let names = list.split("\n").collect::<Vec<&str>>();
    for name in names {
        let name = Path::new(name);
        if let Some(name) = name.file_name() {
            println!("{}", name.to_str().unwrap());
        }
    }


    Ok(())
}
