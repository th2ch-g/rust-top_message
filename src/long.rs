use crate::common::*;

pub fn execute(dir_name: &str, message: &str, time: usize, length: usize) {
    // message to list
    let message_list: Vec<String> = process_message_list(message, length);

    common_execute(dir_name, message_list, time, false);
}

fn process_message_list(message: &str, length: usize) -> Vec<String> {
    let mut message_list: Vec<String> = Vec::new();

    let quantity = message.len() / length;
    let residue = message.len() % length;

    if quantity == 0 {
        message_list.push(message.to_string());
        return message_list;
    }

    for i in 0..quantity {
        message_list.push(message[i * length..i * length + length].to_string());
    }

    if residue != 0 {
        message_list.push(message[(quantity - 1) * length + length..].to_string());
    }

    message_list
}
