use std::fmt::Display;

use sequence::{ Sequence, Encodeable, Decodeable };
use fitness::Fitness;
use mutable::Mutable;
use utility::random;

pub struct Population<TObj, TEnc>{
    max_size: usize,
    mutation_chance: u8,
    pool: Vec<Sequence<TObj, TEnc>>
}

impl<TObj, TEnc> Population<TObj, TEnc>
where   TObj: Encodeable<Vec<TEnc>> + Fitness + Display + Default,
        TEnc: Clone + PartialEq,
        Vec<TEnc>: Decodeable<TObj>,
        Sequence<TObj, TEnc>: Mutable + Ord {

    pub fn new(size: usize, mutation_chance: u8) -> Population<TObj, TEnc> {
        info!("Creating population of size {}, mutation chance: {}%", size, mutation_chance);
        let mut population = Population {
            max_size: size,
            mutation_chance: mutation_chance,
            pool: Vec::new()
        };

        population.increase(size);

        population
    }

    pub fn cycle_once(&mut self) {
        let mut child = {
            let parents = self.select_parents();
            Sequence::crossover_random(parents)
        };

        if (random(100) as u8) < self.mutation_chance {
            child.mutate();
            child.calculate_fitness();
        }

        let mut in_list = false;
        for seq in &self.pool {
            if seq.get_fitness() < child.get_fitness() {
                break;
            }
            if child == *seq {
                in_list = true;
                break;
            }
        }

        if !in_list {
            self.pool.push(child);
            self.sort();
            self.cut();
        }
    }

    pub fn cycle(&mut self, times: u32) {
        for _ in 0..times {
            self.cycle_once()
        }
    }

    pub fn get_fittest(&self) -> &Sequence<TObj, TEnc> {
        &self.pool[0]
    }

    pub fn get_max_fitness(&self) -> i32 {
        self.pool[0].get_fitness()
    }

    pub fn get_pool(&self) -> &Vec<Sequence<TObj, TEnc>> {
        &self.pool
    }

    fn select_parents(&self) -> (&Sequence<TObj, TEnc>, &Sequence<TObj, TEnc>) {

        let mut sum: u8 = 0;
        let mut div: u8 = 2;

        let a = random(100) as u8;
        let b = random(100) as u8;

        let mut parent_index: (Option<usize>, Option<usize>) = (None, None);

        for i in 0..self.max_size {
            sum += 100 / div;
            if div < 100 {
                div *= 2;
            }
            else if div > 100 {
                div = 100;
            }


            if !parent_index.0.is_some() && a < sum {
                parent_index.0 = Some(i);
            }
            else if !parent_index.1.is_some() && b < sum {
                parent_index.1 = Some(i);
            }
        }
        match parent_index {
            (Some(first_index), Some(second_index)) => {
                (&self.pool[first_index], &self.pool[second_index])
            },
            _ => unreachable!()
        }
    }

    fn increase(&mut self, count: usize) {
        for _ in 0..count {
            self.pool.push(Sequence::create_random());
        }
        self.sort();
    }

    fn cut(&mut self) {
        self.pool.truncate(self.max_size);
    }

    fn sort(&mut self) {
        self.pool.sort_by(| a, b | b.cmp(a));
    }
}
