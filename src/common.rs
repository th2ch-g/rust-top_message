use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path;
use std::process;
use std::process::Command;
use std::sync::Arc;
use std::thread;

pub fn compile(dir_name: &str, message: &str) {
    Command::new("rustc")
        .arg(&format!("{}/{}", dir_name, "ms.rs"))
        .arg("-o")
        .arg(&format!("{}/{}", dir_name, message))
        .output()
        .expect(&format!("\n[ERROR] Failed to rustc compile\n[ERROR] {} needs rustc\n[ERROR] Please check Rust environment or install Rust https://www.rust-lang.org/tools/install\n",
                         env!("CARGO_PKG_NAME")));
}

pub fn compile2(dir_name: &str, subdir: &str, message: &str) {
    Command::new("rustc")
        .arg(&format!("{}/{}/{}", dir_name, subdir, "ms.rs"))
        .arg("-o")
        .arg(&format!("{}/{}/{}", dir_name, "run", message))
        .output()
        .expect(&format!("\n[ERROR] Failed to rustc compile\n[ERROR] {} needs rustc\n[ERROR] Please check Rust environment or install Rust https://www.rust-lang.org/tools/install\n",
                         env!("CARGO_PKG_NAME")));
}

pub fn record_current_dir() -> String {
    let current_dir = path::PathBuf::from("./");
    let current_dir = fs::canonicalize(current_dir);
    match current_dir {
        Ok(s) => s.to_string_lossy().to_string(),
        Err(_) => {
            println!("[ERROR] Failed to record current directory");
            String::from("Falied to record current directory")
        }
    }
}

pub fn cd(dir_name: &str) {
    let cd_result = env::set_current_dir(dir_name);
    match cd_result {
        Ok(_) => (),
        Err(_) => {
            println!(
                "[ERROR] Cannot cd directory created by {}",
                env!("CARGO_PKG_NAME")
            );
        }
    }
}

pub fn run(dir_name: &str, message: &str) {
    Command::new(format!("{}/{}", dir_name, message))
        .output()
        .expect("\n[ERROR] Failed to run");
}

pub fn rmdir(dir_name: &str) {
    let rmdir_result = fs::remove_dir_all(dir_name);
    match rmdir_result {
        Ok(_) => (),
        Err(_) => {
            println!("[ERROR] Failed to remove directory");
            println!("[ERROR] But continue");
        }
    }
}

pub fn mkdir(dir_name: &str) {
    let mkdir_result = fs::create_dir(dir_name);
    match mkdir_result {
        Ok(_) => (),
        Err(_) => {
            println!("[ERROR] Failed to create directory");
            println!("[ERROR] Check for directories with the same name");
            process::exit(1);
        }
    }
}

pub fn cat_id(dir_name: &str) {
    let mut file = File::create(format!(
        "{}/{}-{}",
        dir_name,
        env!("CARGO_PKG_NAME"),
        "idfile"
    ))
    .unwrap();
    file.write_all(format!("This file is {} dedicated id file", env!("CARGO_PKG_NAME")).as_bytes())
        .unwrap();
}

pub fn cat(dir_name: &str, thread: usize, time: usize) {
    let mut file = File::create(format!("{}/{}", dir_name, "ms.rs")).unwrap();

    file.write_all(
        b"
use std::thread;use std::time::Instant;use std::sync::Arc;
    ",
    )
    .unwrap();
    file.write_all(
        format!(
            "const THREAD: usize = {};const TIME: u64 = {};",
            thread, time
        )
        .as_bytes(),
    )
    .unwrap();
    file.write_all(
        b"
fn main() {
    let start = Arc::new(Instant::now());
    let mut thrs = Vec::new();
    for _ in 0..THREAD {
        let start = Arc::clone(&start);
        thrs.push(thread::spawn(move || {
            loop { if start.elapsed().as_secs() >= TIME { break } }
        }));
    }
    thrs.into_iter().for_each(|h| h.join().unwrap());
}
                   ",
    )
    .unwrap();
}

//======================================================================

pub fn common_execute(dir_name: &str, message_list: Vec<String>, time: usize, single_bool: bool) {
    // data access for thread
    let dir_name_t = Arc::new(dir_name.to_string().clone());
    let time_t = Arc::new(time.clone());
    let message_list_t = Arc::new(message_list.clone());

    // mkdir
    mkdir(dir_name);
    mkdir(&format!("{}/{}", dir_name, "run"));

    // create file & additional dir & compile
    cat_id(dir_name);

    let mut thrs = Vec::new();
    let mut count = message_list.len();
    for i in 0..message_list.len() {
        let dir_name_r = Arc::clone(&dir_name_t);
        let time_r = Arc::clone(&time_t);
        let message_list_r = Arc::clone(&message_list_t);
        if single_bool {
            thrs.push(thread::spawn(move || {
                mkdir(&format!("{}/{}", dir_name_r, i));
                cat(&format!("{}/{}", dir_name_r, i), 1, *time_r);
                compile2(&dir_name_r, &i.to_string(), &message_list_r[i]);
            }));
        } else {
            thrs.push(thread::spawn(move || {
                mkdir(&format!("{}/{}", dir_name_r, i));
                cat(&format!("{}/{}", dir_name_r, i), count, *time_r);
                compile2(&dir_name_r, &i.to_string(), &message_list_r[i]);
            }));
        }
        count -= 1;
    }
    thrs.into_iter().for_each(|h| h.join().unwrap());

    // record current dir
    let current_dir = record_current_dir();
    cd(&format!("{}/{}", dir_name, "run"));

    // run
    let mut thrs = Vec::new();
    for i in 0..message_list.len() {
        let message_list_r = Arc::clone(&message_list_t);
        thrs.push(thread::spawn(move || {
            run(".", &message_list_r[i]);
        }));
    }
    thrs.into_iter().for_each(|h| h.join().unwrap());

    // cd parent dir
    cd(&current_dir);

    // rmdir
    rmdir(dir_name);
}
