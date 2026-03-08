use clap::Parser;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    source: PathBuf,
    dest: PathBuf,
    #[arg(short, long)]
    force: bool,
}

fn main() -> io::Result<()> {
    let args: Args = Args::parse();
    if !args.source.exists() {
        eprintln!("❌ Error: The source file '{:?}' was not found.", args.source);
        return Ok(());
    }

    let mut final_dest: PathBuf = args.dest.clone();
    if args.dest.is_dir() {
        if let Some(name) = args.source.file_name() {
            final_dest.push(name);
        }
    }

    if final_dest.exists() && !args.force {
        println!("⚠️  Warning: A file with the same name '{:?}' already exists at the destination!", final_dest);
        print!("Selection (y: overwrite / b: backup / n: cancel): ");
        match io::stdout().flush() {
            Ok(_) => {},
            Err(e) => {
                eprintln!("❌ Error: {}", e);
                return Ok(());
            }
        };
        let mut input: String = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("❌ Error: {}", e);
                return Ok(());
            }
        };
        let choice: String = input.trim().to_lowercase();

        if choice == "n" { return Ok(()); }
        if choice == "b" {
            let bak: PathBuf = final_dest.with_extension("bak");
            match fs::rename(&final_dest, &bak) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("❌ Error: {}", e);
                    return Ok(());
                }
            };
            println!("✅ Old file saved as {:?}.", bak);
        }
    }

    match fs::rename(&args.source, &final_dest) {
        Ok(_) => println!("✅ Copy completed successfully"),
        Err(e) if e.kind() == io::ErrorKind::CrossesDevices => {
            match fs::copy(&args.source, &final_dest) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("❌ Error: {}", e);
                    return Ok(());
                }
            };
            println!("✅ Copy completed (Cross-device)");
        }
        Err(e) => eprintln!("❌ Error: {}", e),
    }
    Ok(())
}