use rtm::arg::*;
use rtm::rtm_run;

fn main() {
    let cli = MainArg::new();
    rtm_run(&cli);
    println!("{} done", env!("CARGO_PKG_NAME"));
}
