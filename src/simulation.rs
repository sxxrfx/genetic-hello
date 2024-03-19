use rand::seq::IteratorRandom;

use crate::candidate::Candidate;

const LETTERS: &'static str = "abcdefghijklmnopqrstuvwxyz ";

pub struct Simulation {
    target_str: String,
    target_str_len: u32,
    population_size: u32,
    num_fit_to_keep: u32,
    num_columns: u32,
    column_width: u32,
    mutation_prob: f32,

    population: Vec<Candidate>,
}

impl Simulation {
    pub fn new(
        target_str: &str,
        population_size: u32,
        num_fit_to_keep: u32,
        num_columns: u32,
        mutation_prob: f32,
    ) -> Self {
        Self {
            target_str: target_str.to_owned(),
            target_str_len: target_str.len() as u32,
            population_size,
            num_fit_to_keep,
            num_columns,
            column_width: target_str.len() as u32 + 6,
            mutation_prob,
            population: Vec::new(),
        }
    }

    fn seed_population(&mut self) {
        let mut rng = rand::thread_rng();
        while self.population.len() < self.population_size as usize {
            let mut s = String::new();
            for _ in 0..self.target_str_len {
                s.push(LETTERS.chars().choose(&mut rng).unwrap())
            }
            self.population.push(Candidate::new(&s, true));
        }
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        loop {
            // seed
        }

        Ok(())
    }
}
