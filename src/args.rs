use clap::Parser;

/// Run react native app on ios or android
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Run iOS
    #[arg(short, long)]
    pub ios: bool,

    /// Run Android
    #[arg(short, long)]
    pub android: bool,

    // Simulator
    #[arg(short, long)]
    pub simulator: Option<String>,

    /// Clean install
    #[arg(short, long)]
    pub clean_install: bool,
}