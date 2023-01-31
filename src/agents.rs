use rand::{Rng, thread_rng};

const MAX_LEVEL: i8 = 10;
// const MAX_STAT_LEVEL = 10;

#[derive(Clone)]
pub struct RpgAgent {
    stats: [i8; 3],
}

impl RpgAgent {
    fn enforce_level_max(&mut self) {
        let mut rng = thread_rng();
        while self.stats.iter().sum::<i8>() > MAX_LEVEL {
            let stat_drop = rng.gen_range(0..3);
            self.stats[stat_drop] = (self.stats[stat_drop]-1).max(0);
        }
    }
    pub fn get_stats(&self) -> &[i8; 3] {
        &self.stats
    }
    pub fn readout_stats(self) -> [i8; 3] {
        self.stats
    }
    pub fn set_stat(&mut self, val: i8, index: usize) {
        self.stats[index] = val;
    }
    pub fn level_random(&mut self) {
        let mut rng = thread_rng();
        let stat_boost = rng.gen_range(0..3);
        self.stats[stat_boost] = (self.stats[stat_boost] + 1).min(10);
        self.enforce_level_max();
    }
    pub fn spawn_agent(&self) -> RpgAgent {
        let mut rng = thread_rng();
        let mut new_agent = self.clone();
        let stat_boost = rng.gen_range(0..3);
        let stat_change = rng.gen_range(0..3);
        new_agent.stats[stat_boost] = (new_agent.stats[stat_boost] + stat_change-1).max(0).min(10);
        new_agent.enforce_level_max();
        new_agent
    }
}

pub fn make_agent() -> RpgAgent {
    RpgAgent {
        stats: [0, 0, 0],
    }
}

pub fn make_agent_uniform(level: i8) -> RpgAgent {
    RpgAgent { 
        stats: [level, level, level], 
    }
}




