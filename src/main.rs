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

    /// Use traditional greeting
    #[structopt(short, long)]
    traditional: bool,

    /// Use TEXT as the greeting message
    #[structopt(short, long, value_name = "TEXT", default_value = "Hello, world!")]
    greeting: String,
}

fn init(args: &Args) {
    if let Some(level) = args.verbose.log_level() {
        env_logger::builder()
            .filter_level(level.to_level_filter())
            .write_style(env_logger::fmt::WriteStyle::Always)
            .init();
    }
}

fn hello(args: &Args) {
    debug!("{:?}", args);

    if args.traditional {
        debug!("Printing traditional greeting");
        println!("hello, world");
    } else {
        debug!("Printing non-traditional greeting");
        println!("{}", args.greeting);
    }
}

fn main() -> Result<()> {
    let args = Args::from_args();

    init(&args);
    hello(&args);

    Ok(())
}
