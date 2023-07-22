use rtm_lib::rtm_run;
use rtm_lib::arg::*;

fn main() {
    let cli: MainArg = arg();
    rtm_run(&cli);
    println!("{} done", env!("CARGO_PKG_NAME"));
}
