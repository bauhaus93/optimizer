extern crate genetic;
extern crate rand;

use std::fmt;

use rand::Rng;

use genetic::population::Population;
use genetic::problem::Problem;
use genetic::solution::Solution;
use genetic::utility::{ random, random_100 };

struct EvenSequence {
}

struct EvenEncoding {
    data: Vec<u8>
}

fn fitness(encoding: &EvenEncoding) -> u32 {
    encoding.get().iter().filter(| &e | *e % 2 == 0).count() as u32
}

impl EvenSequence {

    pub fn new() -> EvenSequence {
        EvenSequence { }
    }
}

impl EvenEncoding {

    pub fn new() -> EvenEncoding {
        let mut data: Vec<u8> = Vec::new();
        for _ in 0..1000 {
            data.push(rand::thread_rng().gen::<u8>() % 10)
        }
        EvenEncoding {
            data: data
        }
    }

    pub fn new_from_slice(slice: &[u8]) -> EvenEncoding {
        EvenEncoding {
            data: slice.to_vec()
        }
    }

    pub fn get(&self) -> &[u8] {
        &self.data
    }

    pub fn get_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }
}

impl PartialEq for EvenEncoding {
    fn eq(&self, other: &Self) -> bool {
        self.data.eq(&other.data)
    }
}

impl fmt::Display for EvenEncoding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        self.data.iter().for_each(| e | output += &e.to_string());
        write!(f, "{}", output)
    }
}

impl Problem<EvenEncoding> for EvenSequence {

    fn create_random_solution(&self) -> Solution<EvenEncoding> {
        let encoding = EvenEncoding::new();
        let fitness = fitness(&encoding);
        Solution::new(encoding, fitness)
    }

    fn create_child_solution(&self, parents: (&Solution<EvenEncoding>, &Solution<EvenEncoding>)) -> Solution<EvenEncoding> {
        let encoding_a = parents.0.get_encoding().get();
        let encoding_b = parents.1.get_encoding().get();
        assert!(encoding_a.len() == encoding_b.len());

        let split_point: usize = 1 + random(encoding_a.len() as u32 - 2) as usize;

        let mut child_encoding = Vec::new();
        child_encoding.extend_from_slice(&encoding_a[..split_point]);
        child_encoding.extend_from_slice(&encoding_b[split_point..]);

        let child = EvenEncoding::new_from_slice(&child_encoding);

        let child_fitness = fitness(&child);

        Solution::new(child, child_fitness)
    }

    fn mutate_solution(&self, solution: Solution<EvenEncoding>) -> Solution<EvenEncoding> {
        let mut encoding = solution.consume();

        for e in &mut encoding.get_mut().iter_mut() {
            if random_100() < 20 {
                *e = (*e + 1) % 10;
            }
        }
        let mutated_fitness = fitness(&encoding);
        Solution::new(encoding, mutated_fitness)
    }

}

#[test]
fn test_even_sequence() {
    let problem = EvenSequence::new();
    let mut population = Population::new(&problem, 20, 10);

    population.cycle(100000);
    println!("population info: {}", population);
    assert!(5 < 3);
}
