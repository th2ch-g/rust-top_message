use std::env;
use std::fs;
use std::path;
use std::process;
use std::process::Command;

pub fn execute(onlycheck: bool, onlyrustcheck: bool, onlydircheck: bool, rmcheck: bool) {
    if rmcheck {
        rustc_check();
        let dirs = search_iddir();

        if dirs.is_empty() {
            println!("[CHECK] remove process was not executed");
            process::exit(0);
        }

        rm_iddir(dirs);
    } else if onlycheck {
        rustc_check();
        let dirs = search_iddir();

        if !dirs.is_empty() {
            println!(
                "[CHECK] If you want to delete these directory, please execute --rmcheck option"
            );
        }
    } else if onlyrustcheck {
        rustc_check();
    } else if onlydircheck {
        let dirs = search_iddir();

        if !dirs.is_empty() {
            println!(
                "[CHECK] If you want to delete these directory, please execute --rmcheck option"
            );
        }
    }

    process::exit(0);
}

fn rustc_check() {
    Command::new("rustc")
        .arg("-V")
        .output()
        .expect(&format!("\n[ERROR] rustc not found\n[ERROR] {} needs rustc\n[ERROR] Please check Rust environment or install Rust https://www.rust-lang.org/tools/install\n",
                         env!("CARGO_PKG_NAME")));
    println!("[CHECK] rustc seems to be installed");
}

fn search_iddir() -> Vec<String> {
    let current_dir = path::PathBuf::from(".");

    let files = current_dir.read_dir().unwrap();

    let mut dirs: Vec<String> = Vec::new();

    for file in files {
        let path = file.unwrap().path();

        if path.is_dir() && search_id(&path) {
            let tmp_path = path.file_name().unwrap().to_string_lossy().to_string();
            dirs.push(tmp_path);
        }
    }

    if !dirs.is_empty() {
        for i in 0..dirs.len() {
            println!(
                "[CHECK] {} is seems to be directory created by {}",
                dirs[i],
                env!("CARGO_PKG_NAME")
            );
        }
    } else {
        println!(
            "[CHECK] The diectory created by {} seemed to be absent",
            env!("CARGO_PKG_NAME")
        );
    }

    dirs
}

fn search_id(path: &path::PathBuf) -> bool {
    let files = path.read_dir().unwrap();

    for file in files {
        let subpath = file.unwrap().path();

        if !subpath.is_dir() {
            let tmp_subpath = subpath.file_name().unwrap().to_string_lossy().to_string();
            let id_file_name = format!("{}-idfile", env!("CARGO_PKG_NAME"));

            if tmp_subpath == id_file_name {
                return true;
            }
        }
    }

    false
}

fn rm_iddir(dirs: Vec<String>) {
    for dir in dirs {
        let rmdir_result = fs::remove_dir_all(&dir);

        match rmdir_result {
            Ok(_) => (),
            Err(_) => {
                println!("[ERROR] Failed to remove directory");
                process::exit(1);
            }
        }
    }

    println!(
        "[REMOVED] All directories that {} may have created were successfully deleted",
        env!("CARGO_PKG_NAME")
    );
}
