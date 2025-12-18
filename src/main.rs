use std::env;
use std::process;

mod config;
mod color;
mod clipboard;
mod update;
mod version;
mod gui;
mod tui;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // Handle CLI flags
    if args.len() > 1 {
        match args[1].as_str() {
            "--help" => {
                print_help();
                process::exit(0);
            }
            "--version" => {
                version::print_version();
                process::exit(0);
            }
            "--update" => {
                let channel = args.get(2).map(|s| s.as_str()).unwrap_or("stable");
                match update::check_for_updates(channel) {
                    Ok(has_update) => {
                        if has_update {
                            println!("Update available! Use cascolor --update {} to install.", channel);
                        } else {
                            println!("No update available.");
                        }
                        process::exit(0);
                    }
                    Err(e) => {
                        if matches!(e, update::UpdateError::NotFound) {
                            println!("No update available.");
                            process::exit(0);
                        } else {
                            eprintln!("Update check failed: {}", e);
                            process::exit(1);
                        }
                    }
                }
            }
            _ => {
                eprintln!("Unknown option: {}", args[1]);
                eprintln!("Use --help for usage information");
                process::exit(1);
            }
        }
    }

    // Display detection logic
    let use_tui = should_use_tui();
    let config = config::Config::load();

    if use_tui {
        tui::run(config)?;
    } else {
        gui::run(config)?;
    }
    
    Ok(())
}

fn should_use_tui() -> bool {
    // Check if SSH connection
    if env::var("SSH_CONNECTION").is_ok() || env::var("SSH_CLIENT").is_ok() || env::var("SSH_TTY").is_ok() {
        return true;
    }

    // Check for display environment - must be non-empty
    let has_display = env::var("DISPLAY")
        .ok()
        .filter(|d| !d.is_empty())
        .is_some() 
        || env::var("WAYLAND_DISPLAY")
        .ok()
        .filter(|d| !d.is_empty())
        .is_some();
    
    !has_display
}

fn print_help() {
    println!("cascolor - Beautiful cross-platform color picker

USAGE:
    cascolor [OPTIONS]

OPTIONS:
    --help              Show this help message
    --version           Show version information
    --update [channel]  Check for updates (stable, beta, daily)

DESCRIPTION:
    A beautiful color picker with support for HEX, RGB, RGBA, HSL, HSLA,
    HSV, CMYK, Lab, and more. Features eyedropper tool, color history,
    and cross-platform support.

    Automatically detects display environment:
    - GUI mode: When display is available (X11/Wayland/Windows/macOS)
    - TUI mode: For remote sessions or no display

CONFIG:
    Linux/BSD:  ~/.config/casapps/cascolor/config.toml
    macOS:      ~/Library/Application Support/casapps/cascolor/config.toml
    Windows:    %APPDATA%\\casapps\\cascolor\\config.toml

For more information, visit: https://github.com/casapps/cascolor");
}
