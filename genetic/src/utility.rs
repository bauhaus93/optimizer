use rand;
use rand::Rng;

pub fn random(max: u32) -> u32 {
    rand::thread_rng().gen::<u32>() % max
}

pub fn random_100() -> u8 {
    rand::thread_rng().gen::<u8>() % 100
}
