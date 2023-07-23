use crate::arg::*;
use crate::common::{ mkdir, cat_id, record_current_dir, cd, run, rmdir };
use std::process::Command;

pub fn execute(dir_name: &str, message: &str, gpunum: usize, gpusupport: &GpuSupport, time: usize) {

    let current_dir = record_current_dir();

    // make new directory
    mkdir(dir_name);
    mkdir(&format!("{}/src", dir_name));

    // cat
    cat_id(dir_name);
    cat_toml(dir_name);
    cat_gpu_code();

    // cd
    cd(dir_name);

    // compile
    Command::new("cargo")
        .arg("build")
        .output()
        .expect(&format!("\n[ERROR] Failed to cargo build"));

    // cd2
    cd("./target/debug/");

    // run
    run(".", message);

    // cd back
    cd(&current_dir);

    // rmdir
    rmdir(dir_name);
}


fn cat_toml(dir_name: &str) {

}


fn cat_gpu_code(dir_name: &str, ) {

}


