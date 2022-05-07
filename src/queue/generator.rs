use plotlib::{repr::Plot, style::LineStyle};

use super::{tickable, resource::Resource, rate};

#[derive(Clone, Copy)]
pub struct GeneratorStatistics {
    pub generated_resources: u64
}

impl GeneratorStatistics {

    pub fn generate_plots(vec_server_statistics: &Vec<GeneratorStatistics>) -> Vec<Plot> {
        let mut generated_resources = Vec::new();
        for (i, stats) in vec_server_statistics.iter().enumerate() {
            generated_resources.push((i as f64, stats.generated_resources as f64));
        }
        vec![
            Plot::new(generated_resources).line_style(LineStyle::new().colour("#FF0000")).legend("generated resources".to_string()),
        ]
    }
}

pub trait Generator<R: Resource> : tickable::Tickable {

    fn new(service_rate: rate::Rate) -> Self;
    fn get_plots(&self) -> Vec<Plot>;
    fn generate(&mut self) -> Option<R>;
}
