use crate::common::*;

pub fn execute(dir_name: &str, message_list: &[String], time: usize) {
    // message to list
    let message_list2: Vec<String> = process_message_list(message_list);

    common_execute(dir_name, &message_list2, time, false);
}

fn process_message_list(message_list: &[String]) -> Vec<String> {
    let mut message_list2: Vec<String> = Vec::new();
    let mut tmp = message_list.to_owned();
    tmp.sort_by_key(|x| std::cmp::Reverse(x.len()));
    let maxlen = message_list[0].len();
    for _ in 0..maxlen {
        message_list2.push(String::from(""));
    }
    for t in &tmp {
        for j in 0..maxlen {
            if j < t.len() {
                message_list2[j] += &t[j..j + 1];
            } else {
                message_list2[j] += " ";
            }
        }
    }
    message_list2
}
