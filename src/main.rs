use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(help = "initial game board state")]
    init_state_file: std::path::PathBuf,

    #[arg(
        short = 'r', 
        long,
        default_value_t = 100, 
        value_parser = clap::value_parser!(u64).range(1..=1_000_000),
        help = "delay between iterations in microseconds")]
    refresh_rate_usec: u64,
}

fn main() {
    let args = Args::parse();
    let config = game_of_life::Config::new(
        args.init_state_file,
        args.refresh_rate_usec,
    );

    // if let Err(e) = sierpinski::run_draw_loop(&config) {
    //     eprintln!("error: {}", e);
    //     std::process::exit(1);
    // } 
}
