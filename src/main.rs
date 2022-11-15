#![allow(unused_variables)]
#![allow(dead_code)]

use rand::random;

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
    fn open(&mut self) -> bool {
        true
    }

    fn read(&self, save_to: &mut Vec<u8>) -> usize {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        read_length
    }
    fn close(&mut self) -> bool {
        if random() && random() && random() {
            unsafe {
                ERROR = 1;
            }
        }
        true
    }
}

fn main() {
    let f1_data = vec![114, 117, 115, 116, 33];
    let mut f1 = File::new_with_data("f1.txt", &f1_data);

    let mut buffer: Vec<u8> = vec![];
    f1.open();

    // read
    let f1_length = f1.read(&mut buffer);
    unsafe {
        if ERROR != 0 {
            panic!("An error has occured while reading the file")
        }
    }

    // close
    f1.close();
    unsafe {
        if ERROR != 0 {
            panic!("An error has occured while closing the file")
        }
    }

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f1);
    println!("{} is {} bytes long", &f1.name, f1_length);
    println!("{}", text);
}
