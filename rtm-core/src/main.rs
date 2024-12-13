use rtm_core::arg::*;
use rtm_core::rtm_run;

fn main() {
    let cli: MainArg = arg();
    rtm_run(&cli);
    println!("{} done", env!("CARGO_PKG_NAME"));
}
