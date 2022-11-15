//! Simulating files one step a time.
use rand::prelude::*;

/// Returns true or false to simulate error occurence
fn error_occurs_one_in(denominator: u32) -> bool {
    thread_rng().gen_ratio(1, denominator)
}

#[derive(Debug, PartialEq)]
pub enum FileState {
    Open,
    Closed,
}

/// Represents a "file",
/// which probably lives on a file system.
#[derive(Debug)]
pub struct File {
    pub name: String,
    data: Vec<u8>,
    pub state: FileState,
}

trait Read {
    fn read(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}

impl File {
    /// New files are assumed to be empty, but a name is required.
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    /// Creates new file and initiates it with the given data.
    pub fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    /// returns the file's length in bytes
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns the file's name.
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Opens file for reading
    pub fn open(mut self) -> Result<File, String> {
        if error_occurs_one_in(3) {
            let err_msg = String::from("Permission denied");
            return Err(err_msg);
        }
        self.state = FileState::Open;
        Ok(self)
    }

    /// Closes files. Reading is disabled on closing.
    pub fn close(mut self) -> Result<File, String> {
        if error_occurs_one_in(3) {
            let err_msg = String::from("Interrupted by signal");
            return Err(err_msg);
        }
        self.state = FileState::Closed;
        Ok(self)
    }
}

impl Read for File {
    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading"));
        }
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}

fn main() {
    let f1_data = vec![114, 117, 115, 116, 33];
    let mut f1 = File::new_with_data("f1.txt", &f1_data);

    let mut buffer: Vec<u8> = vec![];
    f1 = f1.open().unwrap();

    // read
    let f1_length = f1.read(&mut buffer).unwrap();

    // close
    f1 = f1.close().unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f1);
    println!("{} is {} bytes long", &f1.name, f1_length);
    println!("{}", text);
}
