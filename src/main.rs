use rtm_lib::arg::*;
use rtm_lib::rtm_run;

fn main() {
    let cli: MainArg = arg();
    rtm_run(&cli);
    println!("{} done", env!("CARGO_PKG_NAME"));
}
