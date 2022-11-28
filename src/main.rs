use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let num_producers = 10;
    for id in 1..=num_producers {
        let tx = tx.clone();
        let _producer = thread::spawn(move || {
            let id_str = format!("p-{}", id);
            if let Err(e) = tx.send(id_str) {
                eprintln!("send error on producer {id}: {e}");
            }
        });
    }

    std::mem::drop(tx);

    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("{}", msg);
        }
    });

    consumer.join().unwrap();
}
