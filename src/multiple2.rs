
use crate::common::{ * };


pub fn execute(dir_name: &str,  message: &str, time: usize) {

    // message to list
    let message_list: Vec<String> = process_message_list(message);

    common_execute(dir_name, message_list, time, true);

}


fn process_message_list(message: &str) -> Vec<String> {

    let tmp: Vec<&str> = message.split_whitespace().collect();

    tmp.iter().map(|x| x.to_string()).collect()

}

