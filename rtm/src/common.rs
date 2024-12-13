use std::io::prelude::*;

pub fn compile(dir_name: &str, message: &str) {
    std::process::Command::new("rustc")
        .arg(format!("{}/{}", dir_name, "ms.rs"))
        .arg("-o")
        .arg(format!("{}/{}", dir_name, message))
        .output()
        .expect("failed to compile");
}

pub fn compile2(dir_name: &str, subdir: &str, message: &str) {
    std::process::Command::new("rustc")
        .arg(format!("{}/{}/{}", dir_name, subdir, "ms.rs"))
        .arg("-o")
        .arg(format!("{}/{}/{}", dir_name, "run", message))
        .output()
        .expect("failed to compile2");
}

pub fn record_current_dir() -> String {
    let current_dir = std::path::PathBuf::from("./");
    let current_dir = std::fs::canonicalize(current_dir);
    match current_dir {
        Ok(s) => s.to_string_lossy().to_string(),
        Err(_) => {
            log::error!("failed to record current directory");
            String::from("err")
        }
    }
}

pub fn cd(dir_name: &str) {
    let cd_result = std::env::set_current_dir(dir_name);
    match cd_result {
        Ok(_) => (),
        Err(_) => {
            log::error!("failed to cd");
        }
    }
}

pub fn run(dir_name: &str, message: &str) {
    std::process::Command::new(format!("{}/{}", dir_name, message))
        .output()
        .expect("failed to run");
}

pub fn rmdir(dir_name: &str) {
    let rmdir_result = std::fs::remove_dir_all(dir_name);
    match rmdir_result {
        Ok(_) => (),
        Err(_) => {
            log::warn!("failed to rmdir but continue");
        }
    }
}

pub fn mkdir(dir_name: &str) {
    let mkdir_result = std::fs::create_dir(dir_name);
    match mkdir_result {
        Ok(_) => (),
        Err(_) => {
            log::error!("failed to create directory");
            log::error!("check authority");
            std::process::exit(1);
        }
    }
}

pub fn cat_id(dir_name: &str) {
    let template = include_str!("template/rtm.idfile");
    let output_path = format!("{}/rtm.idfile", dir_name);
    let mut output_file = std::fs::File::create(&output_path).unwrap();
    output_file.write_all(template.as_bytes()).unwrap();
}

pub fn cat(dir_name: &str, thread: usize, time: usize) {
    let template = include_str!("template/ms.rs");
    let filled_template = template
        .replace("{ thread }", &thread.to_string())
        .replace("{ time }", &time.to_string());
    let output_path = format!("{}/ms.rs", dir_name);
    let mut output_file = std::fs::File::create(&output_path).unwrap();
    output_file.write_all(filled_template.as_bytes()).unwrap();
}

//======================================================================

pub fn common_execute(dir_name: &str, message_list: Vec<String>, time: usize, single_bool: bool) {
    // data access for thread
    let dir_name_t = std::sync::Arc::new(dir_name.to_string().clone());
    let time_t = std::sync::Arc::new(time);
    let message_list_t = std::sync::Arc::new(message_list.clone());

    // mkdir
    mkdir(dir_name);
    mkdir(&format!("{}/{}", dir_name, "run"));

    // create file & additional dir & compile
    cat_id(dir_name);

    let mut thrs = Vec::new();
    let mut count = message_list.len();
    for i in 0..message_list.len() {
        let dir_name_r = std::sync::Arc::clone(&dir_name_t);
        let time_r = std::sync::Arc::clone(&time_t);
        let message_list_r = std::sync::Arc::clone(&message_list_t);
        if single_bool {
            thrs.push(std::thread::spawn(move || {
                mkdir(&format!("{}/{}", dir_name_r, i));
                cat(&format!("{}/{}", dir_name_r, i), 1, *time_r);
                compile2(&dir_name_r, &i.to_string(), &message_list_r[i]);
            }));
        } else {
            thrs.push(std::thread::spawn(move || {
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
        let message_list_r = std::sync::Arc::clone(&message_list_t);
        thrs.push(std::thread::spawn(move || {
            run(".", &message_list_r[i]);
        }));
    }
    thrs.into_iter().for_each(|h| h.join().unwrap());

    // cd parent dir
    cd(&current_dir);

    // rmdir
    rmdir(dir_name);
}
