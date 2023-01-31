
pub trait grid_agent {
    fn char_rep(&self) -> char;
}
pub enum direction {
    N,
    S,
    E,
    W,
    Here,
}
