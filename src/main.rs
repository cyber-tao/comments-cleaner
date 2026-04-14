mod cleaner;
mod constants;
mod language;
mod logger;
mod processor;

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(
    name = "cclean",
    version = env!("CARGO_PKG_VERSION"),
    about = "Code comment cleaning tool - supports C/C++, Java/JavaScript/TypeScript, Python, HTML/CSS, PHP, Rust, Basic",
    long_about = "A powerful command-line tool for automatically removing comments from code in multiple programming languages"
)]
pub struct Cli {
    #[clap(value_name = "PATH", help = "File or directory path to process")]
    pub path: PathBuf,

    #[clap(
        short = 'o',
        long = "output",
        value_name = "OUTPUT",
        help = "Output file path (single file mode) or output directory (directory mode)"
    )]
    pub output: Option<PathBuf>,

    #[clap(
        short = 'r',
        long = "recursive",
        help = "Recursively process all files in directory"
    )]
    pub recursive: bool,

    #[clap(
        short = 'i',
        long = "in-place",
        help = "Modify original file directly (no backup)"
    )]
    pub in_place: bool,

    #[clap(short = 'b', long = "backup", help = "Create backup file (.bak)")]
    pub backup: bool,

    #[clap(
        short = 'l',
        long = "lang",
        value_name = "LANGUAGE",
        help = "Manually specify programming language (c, cpp, java, js, ts, python, html, css, php, rust, basic)"
    )]
    pub language: Option<String>,

    #[clap(long = "dry-run", help = "Dry run, do not actually modify files")]
    pub dry_run: bool,

    #[clap(
        short = 'e',
        long = "extensions",
        value_name = "EXT",
        help = "Specify file extensions to process (comma-separated, e.g.: c,cpp,h)"
    )]
    pub extensions: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    logger::print_header(constants::APP_NAME, constants::APP_VERSION);

    processor::process(&cli.path, &cli)?;

    logger::success(constants::MSG_PROCESSING_COMPLETED);

    Ok(())
}
