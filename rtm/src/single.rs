use crate::common::*;

pub fn execute(dir_name: &str, message: &str, thread: usize, time: usize) {
    // mkdir
    mkdir(dir_name);

    // create file
    cat(dir_name, thread, time);
    cat_id(dir_name);

    // compile
    compile(dir_name, message);

    // record current dir
    let current_dir = record_current_dir();

    // cd tmpdir
    cd(dir_name);

    // run
    run(".", message);

    // cd parent dir
    cd(&current_dir);

    // rmdir
    rmdir(dir_name);
}
