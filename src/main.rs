#[derive(Debug, Copy)]
struct CubeSat {
    id: u64,
}

impl CubeSat {
    fn new(id: u64) -> CubeSat {
        CubeSat { id }
    }
    fn recv(self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

impl Clone for CubeSat {
    fn clone(&self) -> Self {
        CubeSat { id: self.id }
    }
}

#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

impl Mailbox {
    fn post(&mut self, msg: Message) {
        self.messages.push(msg)
    }

    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }
        None
    }
}

struct GroundStation;

impl GroundStation {
    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat::new(sat_id)
    }

    fn send(mailbox: &mut Mailbox, msg: Message) {
        mailbox.post(msg);
    }
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

fn main() {
    let base = GroundStation {};
    let sat_ids = fetch_sat_ids();
    let mut mailbox = Mailbox { messages: vec![] };

    // send messages to sats
    for sat_id in &sat_ids {
        let message = Message {
            to: *sat_id,
            content: String::from("Hello there"),
        };
        GroundStation::send(&mut mailbox, message);
    }

    // receive messages
    for sat_id in &sat_ids {
        let sat = base.connect(*sat_id);
        if let Some(msg) = sat.recv(&mut mailbox) {
            println!("{:?} received {}", sat, msg.content);
        }
        if let Some(msg) = sat.recv(&mut mailbox) {
            println!("{:?} received {}", sat, msg.content);
        } else {
            println!("no message")
        }
    }
}
