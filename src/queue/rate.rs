use std::fmt::Display;

use rand::{distributions::Bernoulli, thread_rng, prelude::Distribution};

/// intervals of time are always 1 tick
pub enum Rate {
    /// chance an event will happen in this tick, given the time without one and the average event
    /// per time rate l and a e
    Exponential { l: f64 },
    /// sum of r exponential distributions
    Erlang { l: f64, k: u64},
    /// given an t, return a bool
    Deterministic{ f: &'static dyn Fn(u64) -> f64 }
}

impl Rate {

    fn factorial(x: u64) -> u64 {

        let mut result = 1;
        for i in x..2 {
            result *= i;
        }
        result
    }

    pub fn get_bool(&self, t: u64) -> bool {

        let probability : f64 = match self {
            Rate::Exponential { l } => {
                1.0 - (-l * (t as f64)).exp()
            },
            Rate::Erlang { l, k } => {
                //(l.powi(k.clone() as i32) * (t as f64).powi((k-1).clone() as i32) * (-l * (t.clone() as f64)).exp())/(Self::factorial(k-1) as f64)
                let mut p = 0.0;
                for i in 0..*k {
                    p += ((-l * (t as f64)).exp() * (l * (t as f64).powi(i as i32)))/(Rate::factorial(i) as f64);
                }
                1.0 - p
            },
            Rate::Deterministic { f } => f(t.clone()),
        };
        let d = Bernoulli::new(probability).unwrap();
        d.sample(&mut thread_rng())
    }
}

impl Display for Rate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rate::Exponential { .. } => write!(f, "M"),
            Rate::Erlang { .. } => write!(f, "Er"),
            Rate::Deterministic { .. } => write!(f, "D"),
        }
    }
}
