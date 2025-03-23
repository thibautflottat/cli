use crate::commands::base::{BaseCommand, CommandBase};

pub struct Command3 {
    base: BaseCommand,
    arg3: String,
    arg4: String,
}

impl Command3 {
    pub fn new(arg1: String, arg3: String, arg4: String) -> Self {
        Self {
            base: BaseCommand::new(arg1),
            arg3,
            arg4,
        }
    }
}

impl CommandBase for Command3 {
    fn base(&self) -> &BaseCommand {
        &self.base
    }

    fn prepare(&self) {
        println!("Preparing Command3 with arg3: {}, arg4: {}", self.arg3, self.arg4);
    }

    fn compute(&self) {
        println!("Computing Command3 with arg3: {}, arg4: {}", self.arg3, self.arg4);
    }
}