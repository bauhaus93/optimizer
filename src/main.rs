#[macro_use]
extern crate log;
extern crate genetic_mkm;

use genetic_mkm::logger;
use genetic_mkm::genetic::population::Population;

use genetic_mkm::genetic::test::*;

pub fn main() {
    match logger::init() {
        Ok(_) => {},
        Err(e) => {
            println!("Could not init logger: {}", e);
            return;
        }
    }

    let mut pop: Population<Test, u8> = Population::new(20, 25);

    for _ in 0..100 {
        pop.cycle(10000);
        //let pool = pop.get_pool();

        info!("Max Fitness: {}", pop.get_max_fitness());

    }

}
