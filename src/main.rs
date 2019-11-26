use clap_verbosity_flag::Verbosity;
use exitfailure::ExitFailure;
use failure::Error;
use indicatif::{ProgressBar, ProgressStyle};
use log::info;
use structopt::clap::AppSettings::ColoredHelp;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, paw_structopt::StructOpt, structopt::StructOpt)]
#[structopt(setting(ColoredHelp))]
pub struct Args {
    #[structopt(flatten)]
    verbose: Verbosity,

    /// Help messages here...
    #[structopt(short = "n", long)]
    count: u64,
}

pub fn init_logger(verbose: &Verbosity) {
    if let Some(level) = verbose.log_level() {
        env_logger::builder()
            .filter_level(level.to_level_filter())
            .init();
    }
}

pub fn progress(len: u64) -> ProgressBar {
    let progress = ProgressBar::new(len);

    progress.set_style(
        ProgressStyle::default_bar()
            .template("{elapsed:>4.cyan} [{bar:40}] {pos:>4}/{len:4} {msg}")
            .progress_chars("=> "),
    );

    progress
}

#[paw::main]
fn main(args: Args) -> Result<(), ExitFailure> {
    init_logger(&args.verbose);

    info!("Start");

    for _ in progress(args.count).wrap_iter(0..args.count) {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ok(())
}
