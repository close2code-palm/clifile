pub use clap::Parser;

#[derive(Parser,Debug)]
#[command(author, version, about, long_about = None)]
pub struct ArgsConfig {
    #[arg(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    pub patterns: Vec<String>,
    #[arg(short, long, default_value_t = true)]
    pub copy: bool,
    #[arg(short, long)]
    pub root: String,
    #[arg(short, long)]
    pub to: String
}