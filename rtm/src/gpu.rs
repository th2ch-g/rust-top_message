use crate::common::*;
use std::io::prelude::*;

pub fn execute(dir_name: &str, message: &str, time: usize) {
    log::info!("GPU checking...");
    env_logger::init();
    pollster::block_on(check_gpu());

    mkdir(dir_name);

    gen_cargo_toml(dir_name, message);

    gen_main_rs(dir_name, time);

    gen_shader_wgsl(dir_name);

    cat_id(dir_name);

    let cwd = record_current_dir();

    cd(dir_name);

    log::info!("Compiling...");

    compile_by_cargo();

    log::info!("Compile done!");

    cd("./target/debug/");

    run(".", message);

    cd(&cwd);

    rmdir(dir_name);
}

async fn check_gpu() {
    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .unwrap();
    dbg!(&adapter.get_info());
    let (_device, _queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Device and Queue"),
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::downlevel_defaults(),
            },
            None,
        )
        .await
        .expect("GPU is not available");
}

fn gen_main_rs(dir_name: &str, time: usize) {
    let template = include_str!("template/gpu/main.rs");
    let filled_template = template.replace("{ time }", &time.to_string());
    let output_path = format!("{}/main.rs", dir_name);
    let mut output_file = std::fs::File::create(&output_path).unwrap();
    output_file.write_all(filled_template.as_bytes()).unwrap();
}

fn gen_cargo_toml(dir_name: &str, message: &str) {
    let template = include_str!("template/gpu/Cargo.toml");
    let filled_template = template.replace("{ name }", &message.to_string());
    let output_path = format!("{}/Cargo.toml", dir_name);
    let mut output_file = std::fs::File::create(&output_path).unwrap();
    output_file.write_all(filled_template.as_bytes()).unwrap();
}

fn gen_shader_wgsl(dir_name: &str) {
    let template = include_str!("template/gpu/shader.wgsl");
    let output_path = format!("{}/shader.wgsl", dir_name);
    let mut output_file = std::fs::File::create(&output_path).unwrap();
    output_file.write_all(template.as_bytes()).unwrap();
}

fn compile_by_cargo() {
    std::process::Command::new("cargo")
        .arg("build")
        .output()
        .expect("failed to cargo build");
}
