mod cli;
mod commands;

use clap::Parser;
use cli::Cli;
use commands::{Command1, Command2, Command3};
use crate::commands::base::CommandBase;
use spdlog::prelude::*;

// fn main() {
//     let cli = Cli::parse();

//     match cli.command {
//         cli::Commands::Command1 { arg1, arg2 } => {
//             let cmd = Command1::new(arg1, arg2);
//             cmd.execute();
//         }
//         cli::Commands::Command2 { arg1 } => {
//             let cmd = Command2::new(arg1);
//             cmd.execute();
//         }
//         cli::Commands::Command3 { arg1, arg3, arg4 } => {
//             let cmd = Command3::new(arg1, arg3, arg4);
//             cmd.execute();
//         }
//     }
// }

fn main() {
    let cli = Cli::parse();

    // Setup logging based on verbosity
    let log_level = match cli.verbose {
        0 => LevelFilter::Equal(Level::Warn),
        1 => LevelFilter::Equal(Level::Info),
        2 => LevelFilter::Equal(Level::Debug),
        _ => LevelFilter::Equal(Level::Trace),
    };

    // Initialize logger with pattern and level
    let logger = spdlog::default_logger();
    logger.set_level_filter(log_level);
    // logger.set_pattern("[%Y-%m-%d %H:%M:%S] [%l] %v");


    let command: Box<dyn CommandBase> = match cli.command {
        cli::Commands::Command1 { arg1, arg2 } => Box::new(Command1::new(arg1, arg2)),
        cli::Commands::Command2 { arg1 } => Box::new(Command2::new(arg1)),
        cli::Commands::Command3 { arg1, arg3, arg4 } => Box::new(Command3::new(arg1, arg3, arg4)),
    };

    command.execute();
}

// fn main() {
//     let cli = Cli::parse();

//     match cli.command {
//         cli::Commands::Command1 { arg1, arg2 } => {
//             let cmd = Command1 { arg1, arg2 };
//             cmd.execute();  // ✅ Now `execute()` is in scope
//         }
//         cli::Commands::Command2 { arg1 } => {
//             let cmd = Command2 { arg1 };
//             cmd.execute();  // ✅ Works now
//         }
//         cli::Commands::Command3 { arg1, arg3, arg4 } => {
//             let cmd = Command3 { arg1, arg3, arg4 };
//             cmd.execute();  // ✅ Works now
//         }
//     }
// }


// like c++

// fn main() {
//     let cli = Cli::parse();

//     match cli.command {
//         cli::Commands::Command1 { arg1, arg2 } => {
//             let cmd = Command1::new(arg1, arg2);
//             cmd.execute();
//         }
//         cli::Commands::Command2 { arg1 } => {
//             let cmd = Command2::new(arg1);
//             cmd.execute();
//         }
//         cli::Commands::Command3 { arg1, arg3, arg4 } => {
//             let cmd = Command3::new(arg1, arg3, arg4);
//             cmd.execute();
//         }
//     }
// }





// fjqdoij 

// mod cli;
// mod commands;

// use clap::Parser;
// use cli::Cli;
// use commands::{command1::Command1, command2::Command2, command3::Command3};
// use crate::commands::base::CommandBase;

// fn main() {
//     let cli = Cli::parse();

//     match cli.command {
//         cli::Commands::Command1 { arg1, arg2 } => {
//             let cmd = Command1::new(arg1, arg2);
//             cmd.execute();
//         }
//         cli::Commands::Command2 { arg1 } => {
//             let cmd = Command2::new(arg1);
//             cmd.execute();
//         }
//         cli::Commands::Command3 { arg1, arg3, arg4 } => {
//             let cmd = Command3::new(arg1, arg3, arg4);
//             cmd.execute();
//         }
//     }
// }