use clap::Parser;
use std::io::{self, Write};
use std::process::Command;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    paths: Vec<PathBuf>,
    #[arg(short = 'r', long)]
    recursive: bool,
    #[arg(short = 'f', long)]
    force: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    if args.paths.is_empty() { return Ok(()); }

    // --- 2段階警告ロジック (前述と同じ) ---
    if args.recursive && args.force {
        println!("⚠️  WARNING: A dangerous deletion operation has been detected!");
        if !confirm("【1/2】Are you sure you want to proceed? (y/n): ")? { return Ok(()); }
        
        print!("【2/2】To execute, enter 'DELETE': ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.trim() != "DELETE" { return Ok(()); }
    }

    // --- OSごとに実行コマンドを切り替え ---
    let status = if cfg!(target_os = "windows") {
        // Windows: PowerShell経由で Remove-Item を実行
        let mut ps_args = vec!["-Command".to_string(), "Remove-Item".to_string()];
        if args.recursive { ps_args.push("-Recurse".to_string()); }
        if args.force { ps_args.push("-Force".to_string()); }
        for p in &args.paths {
            if !p.exists() {
                eprintln!("⚠️  Error: '{}' does not exist. Skipping.", p.display());
                continue; // 存在しないファイルは無視して次へ
            }
            ps_args.push(format!("'{}'", p.display())); // パスにスペースがあってもいいように引用符で囲む
        }
        Command::new("powershell").args(&ps_args).status()
    } else {
        // Linux/WSL: 本物の rm を実行
        let mut cmd = Command::new("rm");
        if args.recursive { cmd.arg("-r"); }
        if args.force { cmd.arg("-f"); }
        cmd.args(&args.paths);
        cmd.status()
    };

    match status {
        Ok(s) if s.success() => println!("✅ Deletion completed successfully"),
        _ => eprintln!("❌ An error occurred while executing the command"),
    }

    Ok(())
}

fn confirm(msg: &str) -> io::Result<bool> {
    print!("{}", msg);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_lowercase() == "y")
}