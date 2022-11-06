use std::io;
use std::io::Write;

fn main() {
    let mut n: u32 = 1;
    let mut divisible_by_3_or_5: bool;
    loop {
        divisible_by_3_or_5 = false;
        if n % 3 == 0 {
            print!("fizz");
            divisible_by_3_or_5 = true;
        }
        if n % 5 == 0 {
            print!("buzz");
            divisible_by_3_or_5 = true;
        }
        if divisible_by_3_or_5 == false {
            print!("{n}");
        }
        print!("\n");
        n = n + 1;
        if n == 20 {
            break;
        }
    }
    io::stdout().flush().unwrap()
}
