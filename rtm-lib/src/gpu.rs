use crate::arg::*;
use crate::common::{ mkdir, cat_id, record_current_dir, cd, run, rmdir };
use std::process::Command;
use std::fs::File;
use std::io::Write;

pub fn execute(dir_name: &str, message: &str, gpunum: usize, gpusupport: &GpuSupport, time: usize) {

    let current_dir = record_current_dir();

    // make new directory
    mkdir(dir_name);
    mkdir(&format!("{}/src", dir_name));

    // cat
    cat_id(dir_name);
    cat_toml(dir_name, message);
    cat_gpu_code(dir_name);

    match gpusupport {
        GpuSupport::Opencl => {
            cat_cl(dir_name);
        }
        GpuSupport::Cuda => {
            cat_fatbin(dir_name);
        }
    }

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


fn cat_toml(dir_name: &str, message: &str) {
    let mut file = File::create(format!("{}/{}", dir_name, "Cargo.toml")).unwrap();
    file.write_all(format!("[package]\nname=\"{}\"\nversion=\"0.1.0\"", message).as_bytes()).unwrap();
    file.write_all(format!("[dependencies]\nrust-gpu-tools = { git = \"https://github.com/filecoin-project/rust-gpu-tools\", rev=\"807fc6c7353b23b9bbeffaafc29e497374423d5d\" }").as_bytes()).unwrap();
}

fn cat_gpu_code(dir_name: &str, ) {

}

fn cat_fatbin(dir_name: &str, ) {

}

fn cat_cl(dir_name: &str) {

}

