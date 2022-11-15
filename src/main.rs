#[derive(Debug)]
enum StatusMessage {
    Ok,
}

type Message = String;

fn check_status(sat: &CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}

impl CubeSat {
    fn new(id: u64) -> CubeSat {
        CubeSat {
            id,
            mailbox: Mailbox { messages: vec![] },
        }
    }
    fn recv(&mut self) -> Option<Message> {
        self.mailbox.messages.pop()
    }
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

struct GroundStation;

impl GroundStation {
    fn send(&self, to: &mut CubeSat, msg: Message) {
        to.mailbox.messages.push(msg)
    }
}

fn main() {
    let base = GroundStation {};
    let mut sat_a = CubeSat::new(0);
    base.send(&mut sat_a, Message::from("ping"));
    println!("sat_a: {:?}", sat_a);

    let msg = sat_a.recv();
    println!("sat_a: {:?}", sat_a);
    println!("msg: {:?}", msg);
}
