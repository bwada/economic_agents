use crate::grid::direction;
use rand;


pub trait creature {
    fn will_replicate(&self) -> bool;
    fn replicate(&self) -> Self;
    fn walk(&self) -> direction;
    fn char_rep(&self) -> char;
}

pub trait autotroph {}

pub trait heterotroph {
    fn starve(&self) -> bool;
}

pub struct red_plant {

}
impl red_plant{
    const REP_PROB: f32 = 0.5;
}
impl creature for red_plant {
    fn will_replicate(&self) -> bool {
        if rand::random::<f32>() < red_plant::REP_PROB {
            return true
        }
        return false
    }
    fn replicate(&self) -> Self {
        return red_plant {}
    }
    fn walk(&self) -> direction {
        return direction::Here
    }
    fn char_rep(&self) -> char {
        return 'r'
    }
}
impl autotroph for red_plant {}



// pub struct blue_plant {

// }
// impl creature for blue_plant {
//     fn replicate(&self) -> bool {
//         let prob = 0.5;
//         if rand::random() < prob {
//             return true
//         }
//         return false
//     }
//     fn walk(&self) -> direction {
//         return direction::Here
//     }
//     fn char_rep(&self) -> char {
//         return 'b'
//     }
// }
// impl autotroph for blue_plant {}

// pub struct red_eater {
//     hunger: u16,
// }
// impl red_eater {
//     const hunger_threshhold: u16 = 100;
// }
// impl creature for red_eater {
//     fn replicate(&self) -> bool {
//         if self.hunger > self.hunger_threshhold {
//             return red_eater {hunger: 50}
//         }
//     }
//     fn walk(&self) -> direction {
//         return direction::Here
//     }
//     fn char_rep(&self) -> char {
//         return 'b'
//     }
// }

// pub struct eater_eater {

// }
// impl creature for eater_eater {
    
// }