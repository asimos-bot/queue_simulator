use plotlib::{repr::Plot, style::LineStyle};

use super::{resource::Resource, tickable, rate};

#[derive(Clone, Copy)]
pub struct ServerStatistics {
    pub served_resources: u64,
    pub busy_ticks: u64,
    pub idle_ticks: u64,
}

impl ServerStatistics {
    pub fn generate_plots(vec_server_statistics: &Vec<ServerStatistics>) -> Vec<Plot> {

        let mut served_resources = Vec::new();
        let mut busy_ticks = Vec::new();
        let mut idle_ticks = Vec::new();
        for (i, stats) in vec_server_statistics.iter().enumerate() {
            served_resources.push((i as f64, stats.served_resources as f64));
            busy_ticks.push((i as f64, stats.busy_ticks as f64));
            idle_ticks.push((i as f64, stats.idle_ticks as f64));
        }
        vec![
            Plot::new(served_resources).line_style(LineStyle::new().colour("#F6FF00")).legend("served resources".to_string()),
            // Plot::new(busy_ticks).line_style(LineStyle::new().colour("#6AFF00")).legend("busy ticks".to_string()),
            // Plot::new(idle_ticks).line_style(LineStyle::new().colour("#00FF80")).legend("idle ticks".to_string())
        ]
    }
}

pub trait Server<R: Resource> : tickable::Tickable {

    fn new(service_rate: rate::Rate) -> Self;
    fn get_plots(&self) -> Vec<Plot>;
    fn is_busy(&self) -> bool;
    /// return true if the server took in a new resource
    fn serve(&mut self, resource: Option<&R>) -> bool;
}
