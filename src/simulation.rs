use core::fmt;
use std::{
    fmt::{Debug, Display},
    thread::sleep,
    time::Duration,
};

use anes::{Attribute, ClearBuffer, Color, ResetAttributes, SetAttribute, SetForegroundColor};
use rand::{
    distributions::{uniform::UniformFloat, Uniform},
    rngs::ThreadRng,
    seq::{index, IteratorRandom},
    thread_rng, Rng,
};

use crate::candidate::Candidate;

const LETTERS: &'static str = "abcdefghijklmnopqrstuvwxyz ";
// const TIME_BETWEEN_STEPS: Duration = Duration::from_millis(1_000);
const TIME_BETWEEN_STEPS: Duration = Duration::from_millis(100);
const TIME_BETWEEN_ACTIONS: Duration = Duration::from_millis(10);

pub struct Simulation {
    state: State,
    step: Step,
    iterations: usize,
}

pub struct State {
    target_str: String,
    population_size: usize,
    num_fit_to_keep: usize,
    num_columns: usize,
    column_width: usize,
    mutation_prob: f32,

    population: Vec<Candidate>,
}

#[derive(PartialEq, Eq)]
pub enum Step {
    SeedPopulation,
    ComputeFitness,
    OrderByFitness,
    RemoveUnfit,
    BreedNew,
    SimulatinEnd,
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Step::SeedPopulation => write!(f, "Seeding the population"),
            Step::ComputeFitness => write!(f, "Computing fitness"),
            Step::OrderByFitness => write!(f, "Sorting by fitness"),
            Step::RemoveUnfit => write!(f, "Removing unfit candidates"),
            Step::BreedNew => write!(f, "Creating new candidates"),
            Step::SimulatinEnd => write!(f, "End of the Simulation Reached!!"),
        }
    }
}

impl State {
    fn seed_population(&mut self) {
        let mut rng = rand::thread_rng();
        while self.population.len() < self.population_size {
            let mut s = String::new();
            for _ in 0..self.target_str.len() {
                s.push(LETTERS.chars().choose(&mut rng).unwrap())
            }
            self.population.push(Candidate::default(&s));
        }
    }
    fn display(&self, label: &str) {
        let padding = self.column_width - self.target_str.len() - 2;

        let mut j = 0;
        let mut mutiple = 1;
        let mut num_rows = self.population.len() / self.num_columns;
        let cells: Vec<String> = Vec::from_iter((0..self.population_size).map(|_| {
            let s = format!(
                // "{:02}{}{}",
                "{}{}",
                // j,
                self.population
                    .get(j)
                    .unwrap()
                    .display_str(&self.target_str),
                std::iter::repeat(" ").take(padding).collect::<String>(),
            );
            if j + num_rows >= self.population.len() {
                j = j % num_rows + 1;
            } else {
                j = j + num_rows;
            }
            s
        }));
        let mut out = String::new();

        for row in cells.chunks(self.num_columns) {
            out = format!("{}   {}\n", out, row.concat());
        }

        println!(
            "\n\n{}{}{:^n$}{}{}\n\n{}\n",
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Cyan),
            label,
            SetForegroundColor(Color::Default),
            ResetAttributes,
            out,
            n = self.column_width * self.num_columns
        );
    }
}

impl Simulation {
    pub fn new(
        target_str: &str,
        population_size: usize,
        num_fit_to_keep: usize,
        num_columns: usize,
        mutation_prob: f32,
    ) -> Self {
        Self {
            state: State {
                target_str: target_str.to_owned(),
                population_size,
                num_fit_to_keep,
                num_columns,
                column_width: target_str.len() + 8,
                mutation_prob,
                population: Vec::new(),
            },
            step: Step::SeedPopulation,
            iterations: 1,
        }
    }
    pub fn run(&mut self) -> std::io::Result<()> {
        loop {
            match self.step {
                Step::SeedPopulation => {
                    self.state.seed_population();
                    self.display();
                    self.step = Step::ComputeFitness;
                }
                Step::ComputeFitness => {
                    self.display();
                    self.step = Step::OrderByFitness;
                }
                Step::OrderByFitness => {
                    self.display();
                    if self.state.population[0].fitness() == self.state.target_str.len() as i32 {
                        self.step = Step::SimulatinEnd;
                    } else {
                        self.step = Step::RemoveUnfit;
                    }
                }
                Step::RemoveUnfit => {
                    self.display();
                    self.step = Step::BreedNew;
                }
                Step::BreedNew => {
                    self.display();
                    self.step = Step::ComputeFitness;
                }
                Step::SimulatinEnd => {
                    self.display();
                    return Ok(());
                }
            }
            self.iterations += 1;
        }
    }

    pub fn display(&mut self) {
        println!("{}", ClearBuffer::All);
        if self.step == Step::SimulatinEnd {
            self.endcard();
            return;
        }
        let label = self.step.to_string();
        match self.step {
            Step::SeedPopulation => {
                self.state.population[0].toggle_focus();
                self.state.population[0].toggle_visibility();
                self.state.display(&label);
                let mut i = 0;
                for j in 1..self.state.population_size {
                    sleep(TIME_BETWEEN_ACTIONS);
                    self.state.population[i].toggle_focus();
                    self.state.population[j].toggle_focus();
                    self.state.population[j].toggle_visibility();
                    self.state.display(&label);
                    i = j;
                }
                sleep(TIME_BETWEEN_ACTIONS);
                self.state.population[self.state.population_size - 1].toggle_focus();
                self.state.display(&label);
            }
            Step::ComputeFitness => {
                let target_str = self.state.target_str.to_owned();
                if self.state.population[0].fitness() < 0 {
                    self.state.population[0].toggle_focus();
                    self.state.population[0].set_fitness(&target_str);
                    self.state.population[0].toggle_fitness_view();
                    self.state.display(&label);
                }
                let mut i = 0;
                for j in 1..self.state.population_size {
                    if self.state.population[j].fitness() < 0 {
                        sleep(TIME_BETWEEN_ACTIONS);
                        self.state.population[i].toggle_focus();
                        self.state.population[j].toggle_focus();
                        self.state.population[j].set_fitness(&target_str);
                        self.state.population[j].toggle_fitness_view();
                        self.state.display(&label);
                    }
                    i = j;
                }
                sleep(TIME_BETWEEN_ACTIONS);
                // self.state.population[self.state.population_size - 1].set_fitness(&target_str);
                self.state.population[self.state.population_size - 1].toggle_focus();
                self.state.display(&label);
            }
            Step::OrderByFitness => {
                let mut made_swap = true;
                let mut evens = true;

                let mut n = 0;
                while made_swap {
                    n += 1;
                    evens = !evens;
                    made_swap = false;

                    let len = self.state.population.len();
                    for i in ((if evens { 1 } else { 0 })..len).step_by(2) {
                        if i + 1 >= len {
                            continue;
                        }
                        let c1 = self.state.population.get(i).unwrap().clone();
                        let c2 = self.state.population.get(i + 1).unwrap().clone();

                        if c1.fitness() >= c2.fitness() {
                            continue;
                        }

                        made_swap = true;
                        *self.state.population.get_mut(i).unwrap() = c2;
                        *self.state.population.get_mut(i + 1).unwrap() = c1;

                        if i % 5 == 0 {
                            self.state.display(&label);
                            sleep(TIME_BETWEEN_ACTIONS);
                        }
                    }
                }
            }
            Step::RemoveUnfit => {
                let num_to_keep = self.state.num_fit_to_keep;
                let mut i = self.state.population_size - 1;

                while i >= num_to_keep {
                    self.state.population[i].toggle_focus();

                    sleep(TIME_BETWEEN_ACTIONS);
                    self.state.display(&label);
                    self.state.population[i].toggle_focus();
                    self.state.population[i].toggle_visibility();

                    i -= 1;
                }
                sleep(TIME_BETWEEN_ACTIONS);
                self.state.population[4].toggle_focus();
                self.state.display(&label);
            }
            Step::BreedNew => {
                let mut rng = thread_rng();
                let num_to_keep = self.state.num_fit_to_keep;
                for i in num_to_keep..self.state.population_size {
                    let indices = index::sample(&mut rng, num_to_keep, 2).into_vec();
                    let parent_a = self.state.population.get_mut(indices[0]).unwrap().text();
                    let parent_b = self.state.population.get_mut(indices[1]).unwrap().text();
                    // make bebe
                    {
                        // focus parent1
                        self.state
                            .population
                            .get_mut(indices[0])
                            .unwrap()
                            .toggle_focus();
                        // focus parent2
                        self.state
                            .population
                            .get_mut(indices[1])
                            .unwrap()
                            .toggle_focus();

                        sleep(TIME_BETWEEN_ACTIONS);
                        self.state.display(&label);

                        self.state.population.get_mut(i).unwrap().toggle_focus();
                        sleep(TIME_BETWEEN_ACTIONS);

                        let mut child = self.breed(parent_a, parent_b, &mut rng);
                        child.toggle_visibility();
                        child.toggle_focus();

                        *self.state.population.get_mut(i).unwrap() = child;
                        self.state.display(&label);
                        self.state.population.get_mut(i).unwrap().toggle_focus();
                        // unfocus parent1
                        self.state
                            .population
                            .get_mut(indices[0])
                            .unwrap()
                            .toggle_focus();
                        // unfocus parent2
                        self.state
                            .population
                            .get_mut(indices[1])
                            .unwrap()
                            .toggle_focus();
                    }
                }
                self.state.display(&label);
            }
            Step::SimulatinEnd => {
                self.state.display(&label);
            }
        }
        sleep(TIME_BETWEEN_STEPS);
    }

    fn breed(&self, parent_a: String, parent_b: String, rng: &mut ThreadRng) -> Candidate {
        let mut child_text = String::new();
        let sample_space = Uniform::new_inclusive(0.0, 1.0);

        let prob_mut = self.state.mutation_prob;
        let prob_a_or_b = 1.0 - prob_mut;
        let prob_a = prob_a_or_b / 2.0;
        for (char_a, char_b) in parent_a.chars().zip(parent_b.chars()) {
            let sample = rng.sample(sample_space);

            if sample > prob_a_or_b {
                child_text.push(LETTERS.chars().choose(rng).unwrap());
            } else {
                if prob_a < sample {
                    child_text.push(char_b);
                } else {
                    child_text.push(char_a);
                }
            }
        }

        Candidate::default(&child_text)
    }

    fn endcard(&self) {
        println!(
            "\n\n{}{}{:^n$}{}{}\n\n",
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Cyan),
            self.step.to_string(),
            SetForegroundColor(Color::Default),
            ResetAttributes,
            n = self.state.column_width * self.state.num_columns
        );
        println!(
            "{}{}{:^n$}{}{}{}",
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Yellow),
            format!(
                "\t     Result: {}{}{}   Iterations: {}{}",
                SetForegroundColor(Color::Green),
                self.state.population[0],
                SetForegroundColor(Color::Yellow),
                SetForegroundColor(Color::White),
                self.iterations
            ),
            SetForegroundColor(Color::Default),
            ResetAttributes,
            std::iter::repeat("\n")
                .take(self.state.population_size / self.state.num_columns - 2)
                .collect::<String>(),
            n = self.state.column_width * self.state.num_columns
        );
    }
}

impl Debug for Simulation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let padding = self.state.column_width - self.state.target_str.len() - 2;

        let cells: Vec<String> = Vec::from_iter((0..self.state.population_size).map(|i| {
            format!(
                "{}{}",
                self.state
                    .population
                    .get(i)
                    .unwrap()
                    .display_str(&self.state.target_str),
                std::iter::repeat(" ").take(padding).collect::<String>()
            )
        }));
        let mut out = String::new();

        for row in cells.chunks(self.state.num_columns) {
            out = format!("{}   {}\n", out, row.concat());
        }

        write!(
            f,
            "\n\n{}{}{:^n$}{}{}\n\n{}\n",
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Cyan),
            self.step.to_string(),
            SetForegroundColor(Color::Default),
            ResetAttributes,
            out,
            n = self.state.column_width * self.state.num_columns
        )?;

        Ok(())
    }
}
