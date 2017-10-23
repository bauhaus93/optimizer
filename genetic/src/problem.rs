use std::fmt;

use solution::Solution;

pub trait Problem<TEnc>
where TEnc: PartialEq + fmt::Display {
    fn create_random_solution(&self) -> Solution<TEnc>;
    fn create_child_solution(&self, parents: (&Solution<TEnc>, &Solution<TEnc>)) -> Solution<TEnc>;
    fn mutate_solution(&self, solution: Solution<TEnc>) -> Solution<TEnc>;
}
