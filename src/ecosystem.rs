use crate::ecosystem_coupling::creature;
use crate::grid;
use crate::ecosystem_coupling;
use array2d::Array2D;


enum EnvCreature {
    None,
    red_plant(ecosystem_coupling::red_plant),
}

pub fn run_eco_1(width: usize, height: usize) {
    let mut grid: Vec<Vec<EnvCreature>> = Vec::new();
    for _i in 0..height {
        grid.push(Vec::new());
        for _j in 0..width {
            grid[0].push(EnvCreature::None)
        }
    }
    grid[0][0] = EnvCreature::red_plant(ecosystem_coupling::red_plant {})
    // let grid = Array2D::filled_with(EnvCreature::No, height, width);
}