use super::{server::{Server, ServerStatistics}, resource::Resource, tickable::Tickable, rate};

pub struct SimpleServer<R> where
R: Resource {
    statistics: Vec<ServerStatistics>,
    distribution: rate::Rate,
    t: u64,
    last_t: u64,
    current_resource: Option<R>
}

impl<R> Server<R> for SimpleServer<R> where
R: Resource {
    fn new(service_rate: rate::Rate) -> Self {
        Self { statistics: Vec::new(), distribution: service_rate, last_t: 0, t: 0, current_resource: None }
    }

    fn get_plots(&self) -> Vec<plotlib::repr::Plot> {
        ServerStatistics::generate_plots(&self.statistics)
    }

    fn is_busy(&self) -> bool {
        self.current_resource.is_some()
    }

    fn serve(&mut self, resource: Option<&R>) -> bool {

        // create statistic object
        let mut statistic = if self.statistics.is_empty() {
            ServerStatistics { served_resources: 0, busy_ticks: 0, idle_ticks: 0 }
        } else {
            self.statistics.last().unwrap().clone()
        };

        let resource_consumed = match (resource, &self.current_resource) {
            (None, None) => {
                statistic.idle_ticks += 1;
                false
            },
            (None, Some(_)) => {
                if self.distribution.get_bool(self.t - self.last_t) {

                    statistic.served_resources += 1;
                    self.current_resource = None;
                }
                statistic.busy_ticks += 1;
                false
            },
            (Some(r), None) => {
                self.current_resource = Some(r.clone());
                statistic.busy_ticks += 1;
                self.last_t = self.t;
                true
            },
            (Some(r_waiting), Some(_)) => {

                statistic.busy_ticks += 1;
                if self.distribution.get_bool(self.t - self.last_t) {
                    statistic.served_resources += 1;
                    self.current_resource = Some(r_waiting.clone());
                    self.last_t = self.t;
                    true
                } else {
                    false
                }
            },
        };
        self.statistics.push(statistic);
        resource_consumed
    }
}

impl<R> Tickable for SimpleServer<R> where
R: Resource {

    /// return current tick valuable
    fn get_tick(&self) -> u64 {
        self.t
    }
    /// increment tick and update entity
    fn tick(&mut self) {
        self.t += 1;
    }
}
