use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let mut handles = vec![];
    let (tx, rx) = mpsc::channel();
    let num_producers = 10;
    let counter = Arc::new(Mutex::new(0));

    for id in 1..=num_producers {
        let tx = tx.clone();
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(1));
            let count = {
                let mut num = counter.lock().unwrap();
                *num += 1;
                num
            };
            let id_str = format!("p-{} -> {}", id, count);
            if let Err(e) = tx.send(id_str) {
                eprintln!("send error on producer {id}: {e}");
            }
        });
        handles.push(handle);
    }

    std::mem::drop(tx);

    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("{}", msg);
        }
    });

    for handle in handles {
        handle.join().unwrap();
    }

    consumer.join().unwrap();
}
