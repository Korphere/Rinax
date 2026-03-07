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
    let args = Args::parse();
    if !args.source.exists() {
        eprintln!("❌ Error: The source file '{:?}' was not found.", args.source);
        return Ok(());
    }

    let mut final_dest = args.dest.clone();
    if args.dest.is_dir() {
        if let Some(name) = args.source.file_name() {
            final_dest.push(name);
        }
    }

    if final_dest.exists() && !args.force {
        println!("⚠️  Warning: A file with the same name '{:?}' already exists at the destination!", final_dest);
        print!("Selection (y: overwrite / b: backup / n: cancel): ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let choice = input.trim().to_lowercase();

        if choice == "n" { return Ok(()); }
        if choice == "b" {
            let bak = final_dest.with_extension("bak");
            fs::rename(&final_dest, &bak)?;
            println!("✅ Old file saved as {:?}.", bak);
        }
    }

    match fs::rename(&args.source, &final_dest) {
        Ok(_) => println!("✅ Move completed successfully"),
        Err(e) if e.kind() == io::ErrorKind::CrossesDevices => {
            fs::copy(&args.source, &final_dest)?;
            fs::remove_file(&args.source)?;
            println!("✅ Move completed (Cross-device)");
        }
        Err(e) => eprintln!("❌ Error: {}", e),
    }
    Ok(())
}