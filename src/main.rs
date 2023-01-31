mod agents;
mod io;
mod simulation;
mod visualizer;

fn main() {
    const NUM_AGENTS: usize = 300;
    let readout = simulation::specialization_1(NUM_AGENTS, 10);
    io::write_2d_arr(readout, "sim_results/specialization_1.txt")

    // visualizer::scatter_3d(readout, "sim_results/test.png")

}
