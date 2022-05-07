use std::marker::PhantomData;

use plotlib::repr::Plot;

use super::{generator::{Generator, GeneratorStatistics}, resource::Resource, tickable::Tickable, rate};

pub struct SimpleGenerator<R> where
R: Resource {

    statistics: Vec<GeneratorStatistics>,
    distribution: rate::Rate,
    t: u64,
    last_t: u64,
    _p: PhantomData<R>,
}

impl<R> Generator<R> for SimpleGenerator<R> where
R: Resource {
    fn new(service_rate: rate::Rate) -> Self {
        Self {statistics: Vec::new(), distribution: service_rate, last_t: 0, t: 0, _p: PhantomData::default() }
    }

    fn get_plots(&self) -> Vec<Plot> {
        GeneratorStatistics::generate_plots(&self.statistics)
    }

    fn generate(&mut self) -> Option<R> {
        // create statistic object
        let mut statistic = if self.statistics.is_empty() {
            GeneratorStatistics { generated_resources: 0 }
        } else {
            self.statistics.last().unwrap().clone()
        };

        let resource = if self.distribution.get_bool(self.t - self.last_t) {
            statistic.generated_resources += 1;
            self.last_t = self.t;
            Some(R::new())
        } else {
            None
        };
        self.statistics.push(statistic);
        resource
    }
}

impl<R> Tickable for SimpleGenerator<R> where
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
