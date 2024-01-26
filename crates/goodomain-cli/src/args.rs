use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, about)]
pub struct Args {
    // Print version and exit
    #[arg(short, long)]
    pub version: bool,

    // Show all TLDs
    #[arg(short, long)]
    pub list: bool,

    // Your word
    #[arg(name = "WORD", required_unless_present_any = &["version", "list"])]
    pub word: Option<String>,
}

pub fn get() -> Args {
    Args::parse()
}
