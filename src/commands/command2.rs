use crate::commands::base::{BaseCommand, CommandBase};

pub struct Command2 {
    base: BaseCommand,
}

impl Command2 {
    pub fn new(arg1: String) -> Self {
        Self {
            base: BaseCommand::new(arg1),
        }
    }
}

impl CommandBase for Command2 {
    fn base(&self) -> &BaseCommand {
        &self.base
    }

    fn prepare(&self) {
        println!("Preparing Command2");
    }

    fn compute(&self) {
        println!("Computing Command2");
    }
}