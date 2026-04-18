use clap::{Parser, Subcommand};
use colored::*;
use std::process::{Command, Stdio};

#[derive(Parser)]
#[command(name = "adb-rs", about = "Fast ADB CLI in Rust")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List connected devices
    Devices,

    /// Get device info
    Info {
        #[arg(short)]
        device: Option<String>,
    },

    /// List installed packages
    Packages {
        #[arg(short)]
        user_only: bool,

        #[arg(short)]
        system_only: bool,

        #[arg(short, long)]
        filter: Option<String>,
    },

    /// Uninstall a package
    Uninstall {
        package: String,

        #[arg(short)]
        device: Option<String>,
    },

    /// Install an APK
    Install {
        apk_path: String,

        #[arg(short)]
        replace: bool,
    },

    /// Push file to device
    Push {
        src: String,
        dst: String,
    },

    /// Pull file from device
    Pull {
        src: String,
        dst: String,
    },

    /// Show device storage
    Storage,

    /// Show battery info
    Battery,

    /// Show running processes
    Ps {
        #[arg(short)]
        filter: Option<String>,
    },

    /// Execute adb shell command
    Shell {
        #[arg(allow_hyphen_values = true)]
        cmd: Vec<String>,
    },
}

fn adb_run(args: &[&str]) -> String {
    let output = Command::new("adb")
        .args(args)
        .output()
        .expect("Failed to run adb");
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn adb_shell(cmd: &str) -> String {
    adb_run(&["shell", cmd])
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Devices => {
            let output = adb_run(&["devices"]);
            println!("{}", output.bright_cyan());
        }

        Commands::Info { device } => {
            let props = vec![
                ("Model", "ro.product.model"),
                ("Android", "ro.build.version.release"),
                ("API Level", "ro.build.version.sdk"),
                ("CPU ABI", "ro.product.cpu.abi"),
                ("Security Patch", "ro.build.version.security_patch"),
                ("Build Type", "ro.build.type"),
                ("Bootloader", "ro.boot.bootloader"),
            ];

            println!("{}", "Device Info".bold().bright_cyan());
            println!("{}", "─".repeat(50).bright_black());

            for (label, prop) in props {
                let val = adb_shell(&format!("getprop {}", prop));
                println!(
                    "  {:<18} {}",
                    label.bright_yellow(),
                    val.trim()
                );
            }

            // Storage
            let storage = adb_shell("df -h /data | tail -1");
            println!("  {:<18} {}", "Storage".bright_yellow(), storage.trim());
        }

        Commands::Packages {
            user_only,
            system_only,
            filter,
        } => {
            let flag = if user_only {
                "-3"
            } else if system_only {
                "-s"
            } else {
                ""
            };

            let output = adb_run(&["shell", "pm", "list", "packages", flag]);
            let mut lines: Vec<_> = output
                .lines()
                .map(|l| l.strip_prefix("package:").unwrap_or(l))
                .collect();

            if let Some(f) = filter {
                lines.retain(|l| l.contains(&f));
            }

            lines.sort();
            println!("{}", format!("Found {} packages\n", lines.len()).bright_green());
            for (i, pkg) in lines.iter().enumerate() {
                if i % 2 == 0 {
                    println!("  {:<40} {}", pkg.bright_white(), "");
                } else {
                    println!("{}", pkg.bright_white());
                }
            }
        }

        Commands::Uninstall { package, device } => {
            print!("Uninstalling {}... ", package.bright_yellow());
            let result = adb_run(&["shell", "pm", "uninstall", "-k", "--user", "0", &package]);
            if result.contains("Success") {
                println!("{}", "✓".bright_green());
            } else {
                println!("{}", "✗".bright_red());
            }
        }

        Commands::Install { apk_path, replace } => {
            let mut args = vec!["install"];
            if replace {
                args.push("-r");
            }
            args.push(&apk_path);
            print!("Installing {}... ", apk_path.bright_yellow());
            let result = adb_run(&args);
            if result.contains("Success") {
                println!("{}", "✓".bright_green());
            } else {
                println!("{}", "✗".bright_red());
            }
        }

        Commands::Push { src, dst } => {
            println!("Pushing {} → {}", src.bright_yellow(), dst.bright_yellow());
            let _output = adb_run(&["push", &src, &dst]);
        }

        Commands::Pull { src, dst } => {
            println!("Pulling {} ← {}", dst.bright_yellow(), src.bright_yellow());
            let _output = adb_run(&["pull", &src, &dst]);
        }

        Commands::Storage => {
            let output = adb_shell("df -h /data /sdcard 2>/dev/null");
            println!("{}", output.bright_cyan());
        }

        Commands::Battery => {
            let fields = vec!["level", "status", "health", "temperature", "voltage"];
            println!("{}", "Battery Status".bold().bright_cyan());
            for field in fields {
                let val = adb_shell(&format!("dumpsys battery | grep {}", field));
                println!("  {}", val.trim());
            }
        }

        Commands::Ps { filter } => {
            let output = adb_shell("ps -A");
            let mut lines: Vec<_> = output.lines().collect();
            if let Some(f) = filter {
                lines.retain(|l| l.contains(&f));
            }
            for line in lines {
                println!("  {}", line.bright_white());
            }
        }

        Commands::Shell { cmd } => {
            let cmd_str = cmd.join(" ");
            let output = adb_shell(&cmd_str);
            println!("{}", output);
        }
    }
}
