use anyhow::Result;
use clap_verbosity_flag::Verbosity;
use log::debug;
use structopt::clap::AppSettings::ColoredHelp;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(setting(ColoredHelp))]
struct Args {
    #[structopt(flatten)]
    verbose: Verbosity,

    /// Output the traditional greeting message "Hello, World!".
    #[structopt(short, long)]
    traditional: bool,

    /// Output text instead of the default greeting.
    #[structopt(short, long, default_value = "Hello, World!")]
    greeting: String,
}

fn init(args: &Args) {
    if let Some(level) = args.verbose.log_level() {
        env_logger::builder()
            .filter_level(level.to_level_filter())
            .init();
    }
}

fn main() -> Result<()> {
    let args = Args::from_args();
    init(&args);

    debug!("{:?}", args);

    if args.traditional {
        println!("Hello, World!");
    } else {
        println!("{}", args.greeting);
    }

    Ok(())
}
