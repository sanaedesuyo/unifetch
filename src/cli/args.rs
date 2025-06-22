use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "unifetch")]
#[command(about = "Windows 11 system information check tool.", long_about = None)]
pub struct Cli {
    /// Display style
    #[arg(short = 's', long = "style", value_enum, default_value_t = DisplayStyle::Default)]
    pub style: DisplayStyle,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum DisplayStyle {
    Default,
    Minimal,
    Detailed,
}