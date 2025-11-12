use anyhow::{Context, Result};
use md_parser_kma::parse_to_html;
use std::env;
use std::fs;
use std::process;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let program_name = args
        .first()
        .cloned()
        .unwrap_or_else(|| "md-parser".to_string());
    let command = args.get(1).map(|s| s.as_str());

    match command {
        Some("parse") => {
            let file_path = args
                .get(2)
                .ok_or_else(|| anyhow::anyhow!("'parse' command requires a file argument"))?;

            let mut output_file = None;
            let mut args_iter = args.iter().skip(3); // skip [program, parse, file]

            while let Some(arg) = args_iter.next() {
                match arg.as_str() {
                    "--output" => {
                        let next_arg = args_iter
                            .next()
                            .ok_or_else(|| anyhow::anyhow!("--output flag requires a filename"))?;
                        output_file = Some(next_arg);
                    }
                    _ => {
                        return Err(anyhow::anyhow!("Unknown argument '{}'", arg));
                    }
                }
            }
            run_parse(file_path, output_file.map(|s| s.as_str()))?;
        }
        Some("help") => {
            print_help();
        }
        Some("credits") => {
            print_credits();
        }
        Some(unknown_cmd) => {
            print_usage(&program_name);
            return Err(anyhow::anyhow!("Unknown command '{}'", unknown_cmd));
        }
        None => {
            print_usage(&program_name);
        }
    }

    Ok(())
}

fn run_parse(file_path: &str, output_file: Option<&str>) -> Result<()> {
    println!("Parsing markdown file: {}", file_path);

    let content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file '{}'", file_path))?;

    println!("Parsing markdown...");
    let html = parse_to_html(&content).with_context(|| "Failed to parse markdown content")?;
    println!("Parse successful!");

    if let Some(output_path) = output_file {
        fs::write(output_path, &html)
            .with_context(|| format!("Error writing to file '{}'", output_path))?;

        println!("HTML saved to: {}", output_path);
    } else {
        println!("\n--- Generated HTML ---");
        println!("{}", html);
        println!("--- End of HTML ---");
    }

    Ok(())
}

fn print_help() {
    println!();
    println!("MD Parser - Markdown to HTML converter");
    println!("---------------------------------------");
    println!();
    println!("COMMANDS:");
    println!("    parse <file> --output <html_file>");
    println!("Parse a markdown file and convert to HTML");
    println!();
    println!("    help");
    println!("Show this help message");
    println!();
    println!("    credits");
    println!("Show project credits and information");
    println!();
    println!("SUPPORTED MARKDOWN:");
    println!("    Headers:         # H1, ## H2, ### H3, etc.");
    println!("    Bold text:       **text**");
    println!("    Italic text:     *text*");
    println!("    Bold & Italic:   ***text***");
    println!("    Unordered lists: - Point or * Point");
    println!("    Ordered lists:   1. Point, 2. Point, etc.");
    println!("    Paragraphs:      Text separated by blank lines");
    println!();
    println!("EXAMPLES:");
    println!("    cargo run parse document.md");
    println!("    cargo run parse document.md --output result.html");
    println!("    cargo run help");
    println!("    cargo run credits");
}

fn print_credits() {
    println!();
    println!("MD Parser");
    println!("---------");
    println!("A simple parser that converts basic Markdown syntax into HTML");
    println!();
    println!("Created by: miqdok");
    println!("Repository: https://github.com/miqdok/md-parser");
    println!();
    println!("Dependencies:");
    println!("  - pest: PEG grammar parser for Rust");
    println!("  - pest_derive: Derive macros for pest");
    println!("  - standard Rust libraries");
    println!();
    println!("Features:");
    println!("  - parsing using PEG grammars");
    println!("  - support for common Markdown elements");
    println!("  - clean HTML output generation");
    println!("  - command-line interface");
    println!("  - test coverage");
    println!();
    println!("2025 - Educational Project");
}

fn print_usage(program_name: &str) {
    eprintln!("Usage: {} <command>", program_name);
    eprintln!("Use '{} help' for available commands", program_name);
}
