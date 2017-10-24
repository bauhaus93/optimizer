use std::fmt;
use std::cmp::Ordering;

pub struct Solution<TEnc>
where TEnc: PartialEq + fmt::Display {
    encoding: TEnc,
    fitness: u32
}

impl<TEnc> Solution<TEnc>
where TEnc: PartialEq + fmt::Display {

    pub fn new(encoding: TEnc, fitness: u32) -> Solution<TEnc> {
        Solution {
            encoding: encoding,
            fitness: fitness
        }
    }

    pub fn get_fitness(&self) -> u32 {
        self.fitness
    }

    pub fn get_encoding(&self) -> &TEnc {
        &self.encoding
    }

    pub fn consume(self) -> TEnc {
        self.encoding
    }
}

impl<TEnc> fmt::Display for Solution<TEnc>
where TEnc: PartialEq + fmt::Display {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.fitness, self.encoding)
    }
}


impl<TEnc> PartialEq for Solution<TEnc>
where TEnc: PartialEq + fmt::Display {
    fn eq(&self, other: &Self) -> bool {
        if self.fitness == other.fitness {
            self.encoding.eq(&other.encoding)
        }
        else {
            false
        }
    }
}

impl<TEnc> Eq for Solution<TEnc>
where TEnc: PartialEq + fmt::Display {
}

impl<TEnc> Ord for Solution<TEnc>
where   TEnc: PartialEq + fmt::Display,
        Solution<TEnc>: Eq {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.fitness == other.fitness {
            true => Ordering::Equal,
            false => {
                match self.fitness < other.fitness {
                    true => Ordering::Less,
                    false => Ordering::Greater
                }
            }
        }
    }
}

impl<TEnc> PartialOrd for Solution<TEnc>
where   TEnc: PartialEq + fmt::Display,
        Solution<TEnc>: Eq {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
