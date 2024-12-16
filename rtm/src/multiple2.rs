use crate::common::*;

pub fn execute(dir_name: &str, message_list: &[String], time: usize) {
    // message to list
    common_execute(dir_name, message_list, time, true);
}
