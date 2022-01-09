use std::ffi::CString;
use std::process;

use nix::sys::wait::*;
use nix::unistd::{execv, fork, ForkResult};

use super::parser;

pub fn execute(root: parser::Command) {
    match root {
        parser::Command::Empty => {}
        parser::Command::Simple { words } => {
            exec(words);
        }
        parser::Command::Pipe {
            upstream,
            downstream,
        } => {}
    }
}

fn exec(words: Vec<String>) {
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => match waitpid(child, None) {
            Ok(WaitStatus::Exited(pid, status)) => {
                println!("CHILD PROCESS EXITED: {:?} {:?}", pid, status);
            }
            Ok(WaitStatus::Signaled(pid, status, _)) => {
                println!("CHILD PROCESS SIGNALED: {:?} {:?}", pid, status);
            }
            Ok(_) => {
                eprintln!("ERROR!");
                process::exit(1);
            }
            Err(err) => {
                eprintln!("ERROR: {}", err);
                process::exit(1);
            }
        },
        Ok(ForkResult::Child) => {
            let args: Vec<CString> = words
                .into_iter()
                .map(|word| CString::new(word).unwrap())
                .collect();
            match execv(&args[0], &args) {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("ERROR: {}", err);
                    process::exit(1);
                }
            }
        }
        Err(err) => {
            eprintln!("ERROR: {}", err);
            process::exit(1);
        }
    }
}
