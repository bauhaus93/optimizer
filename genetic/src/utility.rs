use rand;
use rand::Rng;

pub fn random(max: u32) -> u32 {
    rand::thread_rng().gen::<u32>() % max
}
