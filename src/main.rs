use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (ping_tx, ping_rx) = mpsc::channel();
    let (pong_tx, pong_rx) = mpsc::channel();

    let pinger = thread::spawn(move || {
        let msg = "Ping";
        for _ in 0..10 {
            thread::sleep(Duration::from_secs(1));
            if let Err(e) = ping_tx.send(msg) {
                eprintln!("Send error: {e}");
            }
            match pong_rx.recv() {
                Ok(received) => {
                    println!("{}", received);
                }
                Err(e) => {
                    eprintln!("Receive error: {e}");
                }
            }
        }
    });
    let ponger = thread::spawn(move || {
        let msg = "Pong";
        for _ in 0..5 {
            let received = ping_rx.recv().unwrap();
            println!("{}", received);
            thread::sleep(Duration::from_secs(1));
            pong_tx.send(msg).unwrap();
        }
    });

    pinger.join().unwrap();
    ponger.join().unwrap();
}
