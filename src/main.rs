use simulation::Simulation;

mod candidate;
mod simulation;

fn main() -> std::io::Result<()> {
    let target_str: &str = "hello world";
    let population_size: usize = 48;
    let num_fit_to_keep: usize = 5;
    let num_columns: usize = 4;
    let mutation_prob: f32 = 0.15;

    let mut simulation = Simulation::new(
        target_str,
        population_size,
        num_fit_to_keep,
        num_columns,
        mutation_prob,
    );

    simulation.run()
}
