pub mod arg;
pub mod check;
pub mod common;
pub mod long;
pub mod multiple;
pub mod multiple2;
pub mod single;
pub mod vertical;
pub mod wave;
pub mod gpu;
use crate::arg::*;
use chrono::Utc;
use rand::Rng;
use std::process;

pub fn rtm_run(cli: &MainArg) {
    // process by mode
    match &cli.mode {
        // single mode
        Mode::Single(single_arg) => {
            let dir_name: String = process_tmpdir_name(&single_arg.tmpdir);
            single::execute(
                &dir_name,
                &single_arg.message,
                single_arg.thread,
                single_arg.time,
            );
        }

        // multiple mode
        Mode::Multiple(multiple_arg) => {
            let dir_name: String = process_tmpdir_name(&multiple_arg.tmpdir);
            multiple::execute(
                &dir_name,
                &multiple_arg.message,
                multiple_arg.thread,
                multiple_arg.time,
            );
        }

        // multiple2
        Mode::Multiple2(multiple2_arg) => {
            let dir_name: String = process_tmpdir_name(&multiple2_arg.tmpdir);
            multiple2::execute(&dir_name, &multiple2_arg.message, multiple2_arg.time);
        }

        // long mode
        Mode::Long(long_arg) => {
            let dir_name: String = process_tmpdir_name(&long_arg.tmpdir);
            long::execute(&dir_name, &long_arg.message, long_arg.time, long_arg.length);
        }

        // vertical
        Mode::Vertical(vertical_arg) => {
            let dir_name: String = process_tmpdir_name(&vertical_arg.tmpdir);
            vertical::execute(&dir_name, &vertical_arg.message, vertical_arg.time);
        }

        // wave
        Mode::Wave(wave_arg) => {
            let dir_name: String = process_tmpdir_name(&wave_arg.tmpdir);
            wave::execute(
                &dir_name,
                &wave_arg.message,
                wave_arg.thread,
                wave_arg.length,
            );
        }

        // gpu
        Mode::Gpu(gpu_arg) => {
            let dir_name: String = process_tmpdir_name(&gpu_arg.tmpdir);
            gpu::execute(
                &dir_name,
                &gpu_arg.message,
                gpu_arg.time,
            );
        }

        // check mode
        Mode::Check(check_arg) => {
            check::execute(
                check_arg.onlycheck,
                check_arg.onlyrustcheck,
                check_arg.onlydircheck,
                check_arg.rmcheck,
            );
        }
    }
}

fn process_tmpdir_name(input_name: &str) -> String {
    let default_tmpdir_name = String::from("/tmp/tmp_rtm_(date_randomnumber_pid)");
    if input_name == default_tmpdir_name {
        change_default_tmpdir_name()
    } else {
        input_name.to_string()
    }
}

fn change_default_tmpdir_name() -> String {
    let mut rng = rand::thread_rng();
    let rand_num: u32 = rng.gen();
    format!(
        "{}_{}_{}",
        Utc::now().format("/tmp/tmp_rtm_%Y%m%d%H%M%S"),
        rand_num,
        process::id()
    )
}
