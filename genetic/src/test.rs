use std::str;
use std::fmt;

use sequence::{ Sequence, Encodeable, Decodeable };
use fitness::Fitness;
use mutable::Mutable;
use utility::random;

pub struct Test {
    data: Vec<u8>
}

impl Test {

    pub fn get(&self) -> &Vec<u8> {
        &self.data
    }
}

impl fmt::Display for Test {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Data: [{}]", self.data.iter().fold(String::new(), | acc, a | format!("{}{}", acc, a)))
    }
}

impl Default for Test {

    fn default() -> Test {
        let mut data = Vec::new();
        for _ in 0..100 {
            data.push(random(10) as u8);
        }
        Test {
            data: data
        }
    }
}

impl Fitness for Test {

    fn get_fitness(&self) -> i32 {
        self.data.iter().filter(| e | *e % 2 == 0).count() as i32
    }
}

impl Mutable for Sequence<Test, u8> {
    fn mutate(&mut self) {
        for e in &mut self.get_encoding_mut().iter_mut() {
            if random(100) < 20 {
                *e = (*e + 1) % 10;
            }
        }
    }
}

impl Encodeable<Vec<u8>> for Test {

    fn encode(&self) -> Vec<u8> {
        let mut code = Vec::new();

        for e in &self.data {
            code.push(*e as u8);
        }

        code
    }
}

impl Decodeable<Test> for Vec<u8> {
    fn decode(&self) -> Test {
        let mut data = Vec::new();

        for e in self {
            data.push(*e as u8);
        }

        Test {
            data: data
        }
    }
}
