use plotlib::{repr::Plot, view::ContinuousView, page::Page, style::LineStyle};

use super::{server::Server, client::Client, generator::Generator, tickable::Tickable};


pub enum Policy {
    FIFO,
    FILO
}

#[derive(Clone, Copy)]
struct QueueStatistics {

    discarted_resources: u64,
    queue_size: u64,
}

impl QueueStatistics {
    
    fn get_statistics(vec_queue_statistics: &Vec<QueueStatistics>) -> Vec<Plot> {

        let mut discarted_resources = Vec::new();
        let mut queue_size = Vec::new();

        for (i, stats) in vec_queue_statistics.iter().enumerate() {
            discarted_resources.push((i as f64, stats.discarted_resources as f64));
            queue_size.push((i as f64, stats.queue_size as f64));
        }
        vec![
            Plot::new(discarted_resources).line_style(LineStyle::new().colour("#00FFFB")).legend("discarted resources".to_string()),
            Plot::new(queue_size).line_style(LineStyle::new().colour("#2600FF")).legend("queue size".to_string()),
        ]
    }
}

pub struct Queue<'a, R, G, S> where
R: Client,
G: Generator<R>,
S: Server<R> {

    statistics: Vec<QueueStatistics>,
    queue: Vec<R>,
    server: &'a mut S,
    generator: &'a mut G,
    t: u64,
    capacity: Option<u64>,
    policy: Policy
}

impl<'a, R, G, S> Queue<'a, R, G, S> where
R: Client,
G: Generator<R>,
S: Server<R> {

    pub fn new(generator: &'a mut G, server: &'a mut S, policy: Policy, capacity: Option<u64>) -> Self {

        Queue {
            statistics: Vec::new(),
            queue: Vec::new(),
            server,
            generator,
            t: 0,
            capacity,
            policy
        }
    }
}

impl<'a, R, G, S> Tickable for Queue<'a, R, G, S> where
R: Client,
G: Generator<R>,
S: Server<R> {
    fn get_tick(&self) -> u64 {
        self.t
    }

    fn tick(&mut self) {

        // create statistic object
        let mut statistic = if self.statistics.is_empty() {
            QueueStatistics { discarted_resources: 0, queue_size: 0 }
        } else {
            self.statistics.last().unwrap().clone()
        };

        // generate (or at least try to)
        match self.generator.generate() {
            Some(resource) => {
                match self.capacity {
                    Some(k) if (self.queue.len() as u64) >= k => {
                        statistic.discarted_resources += 1;
                    },
                    None | Some(_) => {
                        match self.policy {
                            Policy::FIFO => self.queue.push(resource),
                            Policy::FILO => self.queue.insert(0, resource)
                        }
                    },
                }
            }
            None => {}
        }

        let next_resource = match self.policy {
            Policy::FIFO => self.queue.first(),
            Policy::FILO => self.queue.last()
        };
        if self.server.serve(next_resource) {
            match self.policy {
                Policy::FIFO => {
                    self.queue.remove(0);
                },
                Policy::FILO => {
                    self.queue.pop();
                }
            }
        }

        statistic.queue_size = self.queue.len() as u64;
        self.statistics.push(statistic);

        self.generator.tick();
        self.server.tick();
        self.t += 1;
    }

}

impl<'a, R, G, S> Queue<'a, R, G, S> where
R: Client,
G: Generator<R>,
S: Server<R> {

    pub fn plot(&self, filename: &str) {
        let mut v = ContinuousView::new();

        for plot in self.generator.get_plots() {
            v = v.add(plot);
        }
        for plot in self.server.get_plots() {
            v = v.add(plot);
        }
        for plot in QueueStatistics::get_statistics(&self.statistics) {
            v = v.add(plot);
        }
        Page::single(&v).save(filename).unwrap();
    }
}
