//! Simulating files one step at a time.

#![allow(dead_code)]

use std::fmt;
use std::fmt::{Display, Formatter};

/// Representing a "file"
/// which probably lives on a file system.
#[derive(Debug, PartialEq)]
pub enum FileState {
    Open,
    Closed,
}

/// Read trait
trait Read {
    /// Reads file data and saves it in save_to
    fn read(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}

#[derive(Debug)]
pub struct File {
    pub name: String,
    data: Vec<u8>,
    pub state: FileState,
}

impl Display for FileState {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl Read for File {
    /// File doesn't do much at this time
    fn read(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        Ok(0)
    }
}

impl File {
    /// New files are assumed to be empty, buy a name is required.
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed
        }
    }

    /// Returns the file's length in bytes
    pub fn ln(&self) -> usize {
        self.data.len()
    }

    /// Returns the file's name.
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

fn main() {
    let f7 = File::new("f7.txt");
    println!("{:?}", f7);
    println!("{}", f7);
}
