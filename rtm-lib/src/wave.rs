use crate::common::*;

use std::sync::Arc;
use std::thread;

pub fn execute(dir_name: &str, message: &str, thread: usize, length: usize) {
    // make message_list
    let message_list = process_message_list(message, length);

    // data access for thread
    let dir_name_t = Arc::new(dir_name.to_string().clone());
    let message_list_t = Arc::new(message_list.clone());

    // mkdir
    mkdir(dir_name);
    mkdir(&format!("{}/{}", dir_name, "run"));

    // create file & additional dir & compile
    cat_id(dir_name);

    let mut thrs = Vec::new();

    for i in 0..message_list.len() {
        let dir_name_r = Arc::clone(&dir_name_t);
        let message_list_r = Arc::clone(&message_list_t);

        thrs.push(thread::spawn(move || {
            mkdir(&format!("{}/{}", dir_name_r, i));
            cat(&format!("{}/{}", dir_name_r, i), thread, 2);
            compile2(&dir_name_r, &i.to_string(), &message_list_r[i]);
        }));
    }

    thrs.into_iter().for_each(|h| h.join().unwrap());

    // record current dir
    let current_dir = record_current_dir();
    cd(&format!("{}/{}", dir_name, "run"));

    // run
    for message in message_list {
        run(".", &message);
    }

    // cd parent dir
    cd(&current_dir);

    // rmdir
    rmdir(dir_name);
}

fn process_message_list(message: &str, length: usize) -> Vec<String> {
    let mut message_list = Vec::new();

    if message.len() < length {
        for i in 0..message.len() {
            let mut tmp = String::from("");

            tmp += &message[i..];
            tmp += &" ".repeat(length - message.len());
            tmp += &message[..i];

            message_list.push(tmp);
        }

        for i in 0..length - message.len() {
            let mut tmp = String::from("");

            tmp += &" ".repeat(length - message.len() - i);
            tmp += message;
            tmp += &" ".repeat(i);

            message_list.push(tmp);
        }
    } else {
        for i in 0..message.len() + 1 {
            let mut tmp = String::from("");
            tmp += &message[i..];
            tmp += " ";
            tmp += &message[..i];

            message_list.push(tmp[..length].to_string());
        }
    }

    message_list
}
