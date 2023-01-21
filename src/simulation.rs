use core::num;
use std::{vec, sync::PoisonError};

use rand_distr::{Normal, Distribution};
use rand::{Rng, thread_rng};
use crate::agents;

fn challenge(agent: &agents::RpgAgent, mean_challenge: f64, var_challenge: f64) -> bool {
    let mut rng = thread_rng();
    let normal = Normal::new(mean_challenge, var_challenge).unwrap();
    let challenge_type = rng.gen_range(0..3);
    let difficulty = normal.sample(&mut rng);
    let mut difficulty = difficulty.round() as i8;
    difficulty = difficulty.max(0);
    difficulty = difficulty.min(10);
    let agent_skill = agent.get_stats()[challenge_type];
    // println!("agent_skill {}", agent_skill);
    // println!("difficulty {}", difficulty);
    // println!("{}", agent_skill >= difficulty);
    agent_skill >= difficulty
}

pub fn specialization_1 (num_agents: usize, iterations: usize) -> Vec<[i8; 3]> {
    let mut rng = thread_rng();
    let mut population: Vec<agents::RpgAgent> = Vec::new();
    for _agent in 0..num_agents{
        population.push(agents::make_agent_uniform(3))
    };
    for _iter in 0..iterations {
        println!("running iteration {}", _iter);
        population.retain(|x| -> bool {challenge(x, 7.0, 2.0)});
        println!("number of survivors {}", population.len());
        if population.len() == 0{
            panic!("Population wipeout")
        }
        while population.len() < num_agents {
            let rand_agent = rng.gen_range(0..population.len());
            population.push(population[rand_agent].spawn_agent())
        }
    };

    let mut return_vec: Vec<[i8; 3]> = Vec::new();
    for agent in population {
        return_vec.push(agent.readout_stats())
    };
    return_vec

}