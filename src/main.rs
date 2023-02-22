use clap::Parser;

/// Run react native app on ios or android
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run iOS
    #[arg(short, long)]
    ios: bool,

    /// Run Android
    #[arg(short, long)]
    android: bool,

    // Simulator
    #[arg(short, long)]
    simulator: Option<String>,

    /// Clean install
    #[arg(short, long)]
    clean: bool,
}

fn run_ios(simulator: Option<String>) {
    println!("Running iOS");
    match simulator {
        Some(x) => { println!("with simulator: {}", x); }
        None => { /* handle the None case */ }
    }
}

fn main() {
    let args = Args::parse();

    if args.ios {
        run_ios(args.simulator);
    }

}