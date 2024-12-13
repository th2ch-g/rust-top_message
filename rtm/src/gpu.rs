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
    let mut file = std::fs::File::create(format!("{}/main.rs", dir_name)).unwrap();
    file.write_all(format!("const TIME: u64 = {};", time).as_bytes())
        .unwrap();
    file.write_all(
        b"
fn main() {
    env_logger::init();
    pollster::block_on(run());
}

async fn run() {
    let start = std::time::Instant::now();
    loop {
        if start.elapsed().as_secs() >= TIME { break }
        let state = State::new().await;
        state.compute();
    }
}

struct State {
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: wgpu::ComputePipeline,
}

impl State {
    async fn new() -> Self {
        let instance = wgpu::Instance::default();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some(\"Device and Queue\"),
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::downlevel_defaults(),
            },
            None
            )
            .await
            .unwrap();

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(\"Shader Module\"),
            source: wgpu::ShaderSource::Wgsl(include_str!(\"shader.wgsl\").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(\"Pipeline Layout\"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some(\"Compute Pipeline\"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: \"main\",
        });

        Self {
            device,
            queue,
            pipeline,
        }
    }

    fn compute(&self) {
        let mut command_encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some(\"Command Encoder\"),
            });

        {
            let mut compute_pass = command_encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some(\"Compute Pass\"),
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&self.pipeline);
            compute_pass.dispatch_workgroups(1, 1, 1);
        }
        self.queue.submit(Some(command_encoder.finish()));
    }
}
",
    )
    .unwrap();
}

fn gen_cargo_toml(dir_name: &str, message: &str) {
    let mut file = std::fs::File::create(format!("{}/Cargo.toml", dir_name)).unwrap();
    file.write_all(
        format!(
            "
[package]
name = \"rtm_gpu\"
version = \"0.1.0\"
edition = \"2021\"

[dependencies]
wgpu = \"0.18\"
pollster = \"0.3.0\"
env_logger = \"0.10.1\"

[[bin]]
name = \"{}\"
path = \"main.rs\"
",
            message
        )
        .as_bytes(),
    )
    .unwrap();
}

fn gen_shader_wgsl(dir_name: &str) {
    let mut file = std::fs::File::create(format!("{}/shader.wgsl", dir_name)).unwrap();
    file.write_all(b"
@workgroup_size(1)
@compute
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    var tmpvec: vec3<f32> = vec3<f32>(1.0, 1.0, 1.0);
    let tmpmat: mat3x3<f32> = mat3x3<f32>(0.707107, -0.707107, 0.0, 0.707107, 0.707107, 0.0, 0.0, 0.0, 1.0);
    loop {
        tmpvec = tmpmat * tmpvec;
    }
}
").unwrap();
}

fn compile_by_cargo() {
    std::process::Command::new("cargo")
        .arg("build")
        .output()
        .expect("failed to cargo build");
}
