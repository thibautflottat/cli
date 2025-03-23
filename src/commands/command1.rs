use crate::commands::base::{BaseCommand, CommandBase};

pub struct Command1 {
    pub base: BaseCommand,  // ✅ Store the base struct
    pub arg2: String,
}

impl Command1 {
    pub fn new(arg1: String, arg2: String) -> Self {
        Self {
            base: BaseCommand::new(arg1),
            arg2,
        }
    }
}

impl CommandBase for Command1 {
    fn base(&self) -> &BaseCommand {
        &self.base  // ✅ This allows access to `arg1`
    }

    fn prepare(&self) {
        println!("Preparing Command1 with arg2: {}", self.arg2);
    }

    fn compute(&self) {
        println!("Computing Command1 with arg2: {}", self.arg2);
    }
}




// fjqdoij

// use crate::commands::base::{BaseCommand, CommandBase};

// pub struct Command1 {
//     base: BaseCommand,  // ✅ Store the base struct
//     arg2: String,
// }

// impl Command1 {
//     pub fn new(arg1: String, arg2: String) -> Self {
//         Self {
//             base: BaseCommand { arg1 },
//             arg2,
//         }
//     }
// }

// impl CommandBase for Command1 {
//     fn base(&self) -> &BaseCommand {
//         &self.base  // ✅ This allows access to `arg1`
//     }

//     fn prepare(&self) {
//         println!("Preparing Command1 with arg2: {}", self.arg2);
//     }

//     fn compute(&self) {
//         println!("Computing Command1 with arg2: {}", self.arg2);
//     }
// }