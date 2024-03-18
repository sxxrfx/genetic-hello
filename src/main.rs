use std::fmt::Display;

use anes::{Color, SetForegroundColor};

const LETTERS: &'static str = "abcdefghijklmnopqrstuvwxyz ";

#[derive(Debug, Clone)]
struct Candidate {
    text: String,
    fitness: i32,
    in_focus: bool,
}

impl Candidate {
    pub fn default(text: &str) -> Self {
        Self {
            text: text.to_owned(),
            fitness: -1,
            in_focus: false,
        }
    }
    pub fn new(text: &str, in_focus: bool) -> Self {
        Self {
            text: text.to_owned(),
            fitness: -1,
            in_focus,
        }
    }
    pub fn toggle_in_focus(&mut self) {
        self.in_focus = !self.in_focus;
    }

    pub fn display_str(&self, target_str: &str) -> String {
        let prefix = if self.in_focus { "➤ " } else { "  " };
        if self.fitness < 0 {
            return format!("{}{}", prefix, self.text);
        }
        let mut out = prefix.to_string();

        for (char, target_char) in self.text.chars().zip(target_str.chars()) {
            if char == target_char {
                out = format!(
                    "{}{}{}",
                    out,
                    SetForegroundColor(Color::Green),
                    char.to_string()
                );
            } else {
                out = format!(
                    "{}{}{}",
                    out,
                    SetForegroundColor(Color::Red),
                    char.to_string()
                );
            }
        }

        format!("{}{}", out, SetForegroundColor(Color::Default))
    }

    pub fn set_fitness(&mut self, target_str: &str) {
        self.fitness = self
            .text
            .chars()
            .zip(target_str.chars())
            .filter(|(a, b)| a == b)
            .count() as i32;
    }
}

fn seed_population(population: &mut Vec<Candidate>, polulation_size: u32, target_str_len: u32) {
    todo!()
}

struct Simulation {
    target_str: String,
    target_str_len: u32,
    population_size: u32,
    num_fit_to_keep: u32,
    num_columns: u32,
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
            mutation_prob,
            population: Vec::new(),
        }
    }
}
fn main() {
    let target_str: &str = "hello world";
    let population_size: u32 = 48;
    let num_fit_to_keep: u32 = 5;
    let num_columns: u32 = 4;
    let mutation_prob: f32 = 0.15;

    let mut simulation = Simulation::new(
        target_str,
        population_size,
        num_fit_to_keep,
        num_columns,
        mutation_prob,
    );

    let mut c1 = Candidate::new("rellr aarad", true);
    let mut c2 = Candidate::new("herro aaaad", false);
    c1.set_fitness(target_str);
    c2.set_fitness(target_str);

    println!("target string:\n  {}", target_str);
    println!("c1:\n{}", c1.display_str(target_str));
    println!("c2:\n{}", c2.display_str(target_str));
}
