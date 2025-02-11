use rtm::arg::*;
use rtm::rtm_run;

fn main() {
    let cli = MainArg::default();
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();
    rtm_run(&cli);
    log::info!("{} done", env!("CARGO_PKG_NAME"));
}
