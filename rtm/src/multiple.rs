use crate::common::*;

pub fn execute(dir_name: &str, message: &str, thread: usize, time: usize) {
    // message to list
    let message_list: Vec<String> = process_message_list(message, thread);

    common_execute(dir_name, message_list, time, true);
}

fn process_message_list(message: &str, thread: usize) -> Vec<String> {
    let mut message_list: Vec<String> = Vec::new();

    for _ in 0..thread {
        message_list.push(message.to_string());
    }

    message_list
}
