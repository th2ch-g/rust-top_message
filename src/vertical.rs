
use crate::common::{ * };

use std::cmp::Reverse;


pub fn execute(dir_name: &str,  message: &str, time: usize) {

    // message to list
    let message_list: Vec<String> = process_message_list(message);

    common_execute(dir_name, message_list, time, false);

}



fn process_message_list(message: &str) -> Vec<String> {


    let mut message_list: Vec<String> = Vec::new();

    let mut tmp: Vec<&str> = message.split_whitespace().collect();

    tmp.sort_by_key(|&x| Reverse(x.len()));


    let maxlen = tmp[0].len();

    for _ in 0..maxlen {

        message_list.push(String::from(""));

    }

    for i in 0..tmp.len() {

        for j in 0..maxlen {

            if j < tmp[i].len() {

                message_list[j] += &tmp[i][j..j+1];

            }
            else {

                message_list[j] += " ";

            }

        }

    }

    message_list

}

