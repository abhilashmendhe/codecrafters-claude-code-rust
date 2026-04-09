use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    #[arg(short = 'p', long)]
    pub prompt: String,
}