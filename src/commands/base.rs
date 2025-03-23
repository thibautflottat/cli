use molly::{XTCReader, Header};
use spdlog::prelude::*;
use std::fs::File;
use std::io::{self, BufWriter, Read, Seek, SeekFrom};
use std::cell::RefCell;

pub struct BaseCommand {
    pub arg1: String,
    pub n_frames: RefCell<usize>,
}

impl BaseCommand {
    pub fn new(arg1: String) -> Self {
        Self {
            arg1,
            n_frames: RefCell::new(0),
        }
    }
}

// pub trait CommandBase {
//     fn base(&self) -> &BaseCommand;
    
//     fn init(&self) -> Result<(), std::io::Error>{

//         // Open trajectory file
//         let file = std::fs::File::open(&self.base().arg1).unwrap_or_else(|err| {
//             eprintln!(
//                 "ERROR: Failed to read trajectory from {:?}: {err}",
//                 &self.base().arg1
//             );
//             std::process::exit(1)
//         });

//         // generate reader
//         let mut reader = XTCReader::new(file);

//         // Generate metadata
//         let offsets = reader.determine_offsets(None)?;
//         let n_frames = offsets.len();
//         *self.base().n_frames.borrow_mut() = n_frames;
//         info!("nframes: {}", n_frames);
        
//         let headers = offsets
//             .iter()
//             .map(|&offset| -> io::Result<Header> {
//                 reader.file.seek(SeekFrom::Start(offset))?;
//                 reader.read_header()
//             })
//             .collect::<io::Result<Vec<_>>>()?;
//         let natoms = headers
//             .first()
//             .map(|header| header.natoms.to_string())
//             .unwrap_or("?".to_string());
//         info!("natoms:  {natoms}");
//         Ok(()) 
//     }

//     fn prepare(&self);

//     fn run(&self) {
//         for i in 0..*self.base().n_frames.borrow() {
//             info!("Running iteration {}", i);
//             self.compute();
//         }
//     }

//     fn compute(&self);

//     fn conclude(&self) {
//         println!("Concluding with arg1: {}", self.base().arg1);
//     }

//     fn execute(&self) {
//         if let Err(e) = self.init() {
//             error!("Initialization failed: {}", e);
//             std::process::exit(1);
//         }
//         self.prepare();
//         self.run();
//         self.conclude();
//     }
// }




// fjqdoij

// pub struct BaseCommand {
//     pub arg1: String,
// }

pub trait CommandBase {
    fn base(&self) -> &BaseCommand;  // ✅ This gives access to `arg1`
    
    fn init(&self) {
        println!("Initializing with arg1: {}", self.base().arg1);
    }

    fn prepare(&self);

    fn run(&self) {
        println!("Running with arg1: {}", self.base().arg1);
        self.compute();
    }

    fn compute(&self);

    fn conclude(&self) {
        println!("Concluding with arg1: {}", self.base().arg1);
    }

    fn execute(&self) {
        self.init();
        self.prepare();
        self.run();
        self.conclude();
    }
}