use queue_simulator::queue::queue::{Queue, Policy};
use queue_simulator::queue::rate::Rate;
use queue_simulator::queue::resource::Resource;
use queue_simulator::queue::server::Server;
use queue_simulator::queue::simple_generator::SimpleGenerator;
use queue_simulator::queue::generator::Generator;
use queue_simulator::queue::simple_server::SimpleServer;
use queue_simulator::queue::tickable::Tickable;

extern crate queue_simulator;

#[derive(Clone, Copy)]
struct Request {
    waiting_ticks: u64,
    served_ticks: u64,
    t: u64
}

impl Resource for Request {
    fn new() -> Self {
        Self { waiting_ticks: 0, served_ticks: 0, t: 0 }
    }

    fn get_waiting_ticks(&self) -> u64 {
        self.waiting_ticks
    }

    fn get_served_ticks(&self) -> u64 {
        self.served_ticks
    }
}

impl Tickable for Request {
    fn get_tick(&self) -> u64 {
        self.t
    }

    fn tick(&mut self) {
        self.t += 1
    }
}

fn main() {
    let mut generator = SimpleGenerator::<Request>::new(Rate::Exponential { l: 0.3 });

    let mut server = SimpleServer::<Request>::new(Rate::Exponential { l: 0.2 });

    let mut queue = Queue::new(&mut generator, &mut server, Policy::FIFO, Some(10));
    for _ in 1..1000 {
        queue.tick();
    }

    queue.plot("hello.svg");
}
