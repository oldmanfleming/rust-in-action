use std::rc::Rc;

struct GroundStation;

impl GroundStation {
    fn send(&self, mailbox: &mut Mailbox, msg: Message) {
        mailbox.post(msg);
    }

    fn connect(&self, id: u64) -> CubeSat {
        CubeSat { id }
    }
}

#[derive(Debug)]
struct CubeSat {
    id: u64,
}

impl CubeSat {
    fn recv(&self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

impl Mailbox {
    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                return Some(self.messages.remove(i));
            }
        }
        None
    }
}

#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

fn main() {
    let ground_station = GroundStation;
    let mut mail = Mailbox { messages: vec![] };

    let sat_ids = Rc::new(fetch_sat_ids());

    for sat_id in sat_ids.iter() {
        let sat = ground_station.connect(*sat_id);
        let msg = Message {
            to: *sat_id,
            content: String::from("hello"),
        };
        ground_station.send(&mut mail, msg);
    }

    for sat_id in sat_ids.iter() {
        let sat = ground_station.connect(*sat_id);

        let msg = sat.recv(&mut mail);
        println!("{:?}: {:?}", sat, msg);
    }
}
