use queue_simulator::queue::queue::{Queue, Policy};
use queue_simulator::queue::rate::Rate;
use queue_simulator::queue::client::Client;
use queue_simulator::queue::server::Server;
use queue_simulator::queue::simple_generator::SimpleGenerator;
use queue_simulator::queue::generator::Generator;
use queue_simulator::queue::simple_server::SimpleServer;
use queue_simulator::queue::tickable::Tickable;

extern crate queue_simulator;

#[derive(Clone, Copy)]
struct Request {
}

impl Client for Request {
    fn new() -> Self {
        Self {} 
    }
}

fn main() {
    let mut generator = SimpleGenerator::<Request>::new(Rate::Exponential { l: 0.4 });

    let mut server = SimpleServer::<Request>::new(Rate::Exponential { l: 0.2 });

    let mut queue = Queue::new(&mut generator, &mut server, Policy::FIFO, Some(10));
    for _ in 1..100 {
        queue.tick();
    }

    queue.plot("hello.svg");
}
