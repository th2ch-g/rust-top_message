use crate::arg::*;
use crate::common::*;

impl TopMessage for MultipleArg {
    fn messages(&self) -> Vec<String> {
        vec![self.message.clone(); self.thread]
    }

    fn dir_name(&self) -> &str {
        &self.dir_name
    }

    fn run(self) {
        self.clone().template_run(self.time, true);
    }
}
