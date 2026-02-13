//! omega-theme: CLI tool for Omega theme developers.
//!
//! Provides validation, accessibility auditing, and conversion utilities.

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use omega_core::color::ColorTheme;
use palette::{Srgb, FromColor, LinLuma};

#[derive(Parser)]
#[command(name = "omega-theme")]
#[command(about = "CLI tool for Omega theme developers", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validates one or more theme files.
    Validate {
        /// Path to a theme file or directory.
        path: PathBuf,
    },
    /// Checks a theme for WCAG contrast compliance.
    CheckContrast {
        /// Path to the theme file.
        path: PathBuf,
        /// Level to check against (aa or aaa).
        #[arg(short, long, default_value = "aa")]
        level: String,
    },
    /// Converts a theme between different color formats (WIP).
    Convert {
        /// Path to the theme file.
        path: PathBuf,
        /// Target format (hex, oklch).
        #[arg(short, long)]
        to: String,
    },
    /// Exports a theme to external formats (e.g. Alacritty).
    Export {
        /// Path to the theme file.
        path: PathBuf,
        /// Target format (alacritty, wezterm).
        #[arg(short, long)]
        format: String,
    },
    /// Previews a theme in the terminal.
    Preview {
        /// Path to the theme file.
        path: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Validate { path } => validate_theme(&path)?,
        Commands::CheckContrast { path, level } => check_contrast(&path, &level)?,
        Commands::Convert { path, to } => convert_theme(&path, &to)?,
        Commands::Export { path, format } => export_theme(&path, &format)?,
        Commands::Preview { path } => preview_theme(&path)?,
    }

    Ok(())
}

fn validate_theme(path: &Path) -> Result<()> {
    if path.is_dir() {
        println!("Validating directory: {}", path.display());
        let mut count = 0;
        let mut failures = 0;
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            if entry.path().extension().is_some_and(|ext| ext == "toml") {
                count += 1;
                if let Err(e) = validate_single_file(&entry.path()) {
                    eprintln!("FAIL: {}: {}", entry.path().display(), e);
                    failures += 1;
                } else {
                    println!("PASS: {}", entry.path().display());
                }
            }
        }
        println!("
Summary: {} themes validated, {} failures", count, failures);
        if failures > 0 {
            std::process::exit(1);
        }
    } else {
        validate_single_file(path)?;
        println!("Theme is valid: {}", path.display());
    }
    Ok(())
}

fn validate_single_file(path: &Path) -> Result<()> {
    let theme = ColorTheme::load_from_file(path)
        .with_context(|| format!("Failed to load theme from {}", path.display()))?;
    
    theme.validate_strict()
        .map_err(|errors| {
            let mut msg = String::from("Strict validation failed:
");
            for err in errors {
                msg.push_str(&format!("  - {}
", err));
            }
            anyhow::anyhow!(msg)
        })?;
    
    Ok(())
}

fn check_contrast(path: &Path, level: &str) -> Result<()> {
    let theme = ColorTheme::load_from_file(path)?;
    let target_ratio = match level.to_lowercase().as_str() {
        "aaa" => 7.0,
        _ => 4.5,
    };

    println!("Checking contrast for theme: {} (Target: {}:1)", theme.meta.name, target_ratio);
    
    let mut failures = 0;

    // Check semantic mappings
    println!("
Semantic Colors:");
    // This is a bit simplified, we should check what they are actually paired with in UI
    // For now, check against theme background (base.black)
    let (_, bg_hex) = theme.resolve_reference("base.black").unwrap_or((
        omega_core::color::HexColor::from_hex("#000000").unwrap(),
        omega_core::color::HexColor::from_hex("#000000").unwrap()
    ));
    
    let check = |name: &str, fg_hex: omega_core::color::HexColor, failures: &mut i32| {
        let ratio = contrast_ratio(fg_hex, bg_hex);
        if ratio < target_ratio {
            println!("  [FAIL] {}: ratio {:.2}:1 (too low)", name, ratio);
            *failures += 1;
        } else {
            println!("  [PASS] {}: ratio {:.2}:1", name, ratio);
        }
    };

    // Check some representative semantic keys
    if let Some((fg, _)) = theme.resolve_reference("semantic.danger") { check("danger", fg, &mut failures); }
    if let Some((fg, _)) = theme.resolve_reference("semantic.success") { check("success", fg, &mut failures); }
    if let Some((fg, _)) = theme.resolve_reference("semantic.info") { check("info", fg, &mut failures); }
    if let Some((fg, _)) = theme.resolve_reference("semantic.warning") { check("warning", fg, &mut failures); }

    if failures > 0 {
        eprintln!("
Accessibility check FAILED with {} contrast issues", failures);
        std::process::exit(1);
    } else {
        println!("
Accessibility check PASSED");
    }

    Ok(())
}

fn contrast_ratio(fg: omega_core::color::HexColor, bg: omega_core::color::HexColor) -> f32 {
    let (fr, fg_g, fb) = fg.to_rgb();
    let (br, bg_g, bb) = bg.to_rgb();
    
    let f_color = Srgb::new(fr as f32 / 255.0, fg_g as f32 / 255.0, fb as f32 / 255.0).into_linear();
    let b_color = Srgb::new(br as f32 / 255.0, bg_g as f32 / 255.0, bb as f32 / 255.0).into_linear();
    
    let f_lum: LinLuma = LinLuma::from_color(f_color);
    let b_lum: LinLuma = LinLuma::from_color(b_color);
    
    let l1 = f_lum.luma.max(b_lum.luma);
    let l2 = f_lum.luma.min(b_lum.luma);
    
    (l1 + 0.05) / (l2 + 0.05)
}

fn convert_theme(_path: &Path, _to: &str) -> Result<()> {
    println!("Convert command is not yet fully implemented.");
    Ok(())
}

fn export_theme(path: &Path, format: &str) -> Result<()> {
    let theme = ColorTheme::load_from_file(path)?;
    match format.to_lowercase().as_str() {
        "alacritty" => export_alacritty(&theme)?,
        _ => println!("Export format '{}' is not supported.", format),
    }
    Ok(())
}

fn export_alacritty(theme: &ColorTheme) -> Result<()> {
    println!("# Alacritty color config for {}", theme.meta.name);
    println!("[colors.primary]");
    let (fg, bg) = theme.resolve_reference("base.white").unwrap();
    println!("foreground = '{}'", fg);
    println!("background = '{}'", bg);
    
    println!("
[colors.normal]");
    let print_base = |name: &str| {
        if let Some((c, _)) = theme.resolve_reference(&format!("base.{}", name)) {
            println!("{} = '{}'", name, c);
        }
    };
    print_base("black");
    print_base("red");
    print_base("green");
    print_base("yellow");
    print_base("blue");
    print_base("magenta");
    print_base("cyan");
    Ok(())
}

fn preview_theme(path: &Path) -> Result<()> {
    let theme = ColorTheme::load_from_file(path)?;
    println!("Previewing Theme: {} by {}", theme.meta.name, theme.meta.author);
    println!("Description: {}\n", theme.meta.description);

    println!("Base Palette:");
    let print_swatch = |name: &str, hex: omega_core::color::HexColor| {
        let (r, g, b) = hex.to_rgb();
        // ANSI escape sequence for background color
        print!("\x1b[48;2;{};{};{}m  \x1b[0m {:<10} {}", r, g, b, name, hex);
    };

    let p = &theme.base;
    print_swatch("black", p.black); print!("  "); print_swatch("red", p.red); println!();
    print_swatch("green", p.green); print!("  "); print_swatch("yellow", p.yellow); println!();
    print_swatch("blue", p.blue); print!("  "); print_swatch("magenta", p.magenta); println!();
    print_swatch("cyan", p.cyan); print!("  "); print_swatch("white", p.white); println!();
    print_swatch("gray", p.gray); println!();

    println!("\nSemantic Samples:");
    if let Some((fg, _)) = theme.resolve_reference("semantic.danger") { print_swatch("danger", fg); println!(); }
    if let Some((fg, _)) = theme.resolve_reference("semantic.success") { print_swatch("success", fg); println!(); }
    if let Some((fg, _)) = theme.resolve_reference("semantic.info") { print_swatch("info", fg); println!(); }
    
    Ok(())
}
