use std::ops::Index;

use crate::ecosystem_coupling::creature;
use crate::grid;
use crate::ecosystem_coupling;
use rand::{Rng, thread_rng};


enum EnvCreature {
    None,
    RedPlant(ecosystem_coupling::red_plant),
}

struct SpawnCount {
    red_plant: usize,
    blue_plant: usize,
    red_eater: usize,
    blue_eater: usize,
    eater_eater: usize,
}
impl Index<EnvCreature> for SpawnCount {
    type Output = usize;
    fn index(&self, index: EnvCreature) -> &Self::Output {
        match index {
            EnvCreature::None => &0,
            EnvCreature::RedPlant(plant) => &self.red_plant,
        }
    }
}

pub fn rand_cell(width: usize, height: usize) -> (usize, usize) {
    let mut rng  = thread_rng();
    let x: usize = rng.gen_range(0..width);
    let y: usize = rng.gen_range(0..height);
    (x,y)
}

fn print_grid(grid: &Vec<Vec<EnvCreature>>) {
    for row in grid {
        for element in row {
            match element {
                EnvCreature::None => {print!(".")}
                EnvCreature::RedPlant(plant) => {print!("{}", plant.char_rep())}
            }
        }
        println!("");
    }
    println!("");
    println!("");
}

fn step_grid(grid: &Vec<Vec<EnvCreature>>) {
    for counter in 0..15 {
        println!("iteration {}", counter);
        print_grid(&grid);
        let mut red_plant_spawn_count = 0;
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                let element = &grid[row][col];
                match element {
                    EnvCreature::None => {},
                    EnvCreature::RedPlant(plant) => {
                        if plant.will_replicate() {
                            red_plant_spawn_count+=1;
                        }
                    },
                }
            }
        }

    }
}

pub fn run_eco_1(width: usize, height: usize) {
    let mut grid: Vec<Vec<EnvCreature>> = Vec::new();
    for i in 0..height {
        grid.push(Vec::new());
        for _j in 0..width {
            grid[i].push(EnvCreature::None)
        }
    }
    grid[0][0] = EnvCreature::RedPlant(ecosystem_coupling::red_plant {});

    for counter in 0..15 {
        println!("iteration {}", counter);
        print_grid(&grid);
        let mut red_plant_spawn_count = 0;
        for row in 0..height {
            for col in 0..width {
                let element = &grid[row][col];
                match element {
                    EnvCreature::None => {},
                    EnvCreature::RedPlant(plant) => {
                        if plant.will_replicate() {
                            red_plant_spawn_count+=1;
                        }
                    },
                }
            }
        }
        for _spawn in 0..red_plant_spawn_count {
            let cell = rand_cell(width, height);
            if let EnvCreature::None = grid[cell.0][cell.1]{
                grid[cell.0][cell.1] = EnvCreature::RedPlant(ecosystem_coupling::red_plant {});
            }
        }
    }
}