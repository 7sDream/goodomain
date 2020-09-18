use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Args {
    // Print version and exit
    #[structopt(short, long)]
    pub version: bool,

    // Show all TLDs
    #[structopt(short, long)]
    pub list: bool,

    // Your word
    #[structopt(name = "WORD", required_unless_one = &["version", "list"])]
    pub word: Option<String>,
}

pub fn get() -> Args {
    Args::from_args()
}
