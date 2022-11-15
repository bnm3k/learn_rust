#![allow(unused_variables)]
#![allow(dead_code)]

use rand::prelude::*;

fn error_occurs_one_in(denominator: u32) -> bool {
    thread_rng().gen_ratio(1, denominator)
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

static mut ERROR: i32 = 0;

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
    fn open(self) -> Result<File, String> {
        if error_occurs_one_in(3) {
            let err_msg = String::from("Permission denied");
            return Err(err_msg);
        }
        Ok(self)
    }
    fn close(self) -> Result<File, String> {
        if error_occurs_one_in(3) {
            let err_msg = String::from("Interrupted by signal");
            return Err(err_msg);
        }
        Ok(self)
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
