use std::fmt;

use problem::Problem;
use solution::Solution;
use utility::random_100;

pub struct Population<'a, TEnc: 'a>
where TEnc: PartialEq {
    problem: &'a Problem<TEnc>,
    max_size: usize,
    mutation_chance: u8,
    pool: Vec<Solution<TEnc>>,
    total_cycles: u32,
}

impl<'a, TEnc> fmt::Display for Population<'a, TEnc>
where TEnc: PartialEq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cycles: {}, fittest: {}, median: {}",
            self.get_cycle_count(),
            self.get_max_fitness(),
            self.get_median_fitness()
        )
    }
}

impl<'a, TEnc> Population<'a, TEnc>
where TEnc: PartialEq {

    pub fn new(problem: &'a Problem<TEnc>, size: usize, mutation_chance: u8) -> Population<TEnc> {
        info!("Creating population of size {}, mutation chance: {}%", size, mutation_chance);
        let mut population = Population {
            problem: problem,
            max_size: size,
            mutation_chance: mutation_chance,
            pool: Vec::new(),
            total_cycles: 0
        };

        population.increase(size);

        population
    }

    pub fn cycle_once(&mut self) {
        let mut child = {
            let parents = self.select_parents();
            self.problem.create_child_solution(parents)
        };

        if random_100() < self.mutation_chance {
            child = self.problem.mutate_solution(child);
        }

        if !self.pool.iter().any(| ref e | *e ==  &child) {
            self.pool.push(child);
            self.sort();
            self.cut();
        }
        self.total_cycles += 1;
        //println!("{}, {}, {}", self.pool[0], self.pool[1], self.pool[2]);
    }

    pub fn cycle(&mut self, times: u32) {
        for _ in 0..times {
            self.cycle_once()
        }
    }

    pub fn get_fittest(&self) -> &Solution<TEnc> {
        &self.pool[0]
    }

    pub fn get_max_fitness(&self) -> u32 {
        self.pool[0].get_fitness()
    }

    pub fn get_median_fitness(&self) -> u32 {
        self.pool[self.pool.len() / 2].get_fitness()
    }

    pub fn get_pool(&self) -> &[Solution<TEnc>] {
        &self.pool
    }

    pub fn get_cycle_count(&self) -> u32 {
        self.total_cycles
    }

    /*pub fn print_population(&self) {
        println!("population:");
        for sol in &self.pool {
            println!("solution: {}", sol);
        }
    }*/

    fn select_parents(&self) -> (&Solution<TEnc>, &Solution<TEnc>) {

        let mut sum: u8 = 0;
        let mut div: u8 = 2;

        let a = random_100();
        let b = random_100();

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
            self.pool.push(self.problem.create_random_solution());
        }
        self.sort();
    }

    fn cut(&mut self) {
        self.pool.truncate(self.max_size);
    }

    fn sort(&mut self) {
        self.pool.sort_by(| a, b | b.cmp(&a));
    }
}
