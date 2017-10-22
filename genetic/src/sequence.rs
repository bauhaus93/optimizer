use std::marker::PhantomData;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Display;


use fitness::Fitness;
use utility::random;

pub trait Encodeable<TEnc> {
    fn encode(&self) -> TEnc;
}

pub trait Decodeable<TObj> {
    fn decode(&self) -> TObj;
}

pub struct Sequence<TObj, TEnc> {
    phantom: PhantomData<TObj>,
    encoding: Vec<TEnc>,
    fitness: i32
}

impl<TObj, TEnc> Display for Sequence<TObj, TEnc>
where   TObj: Display,
        Vec<TEnc>: Decodeable<TObj> {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fitness: {} | {}", self.fitness, self.encoding.decode().to_string())
    }
}

impl<TObj, TEnc> PartialEq for Sequence<TObj, TEnc>
where   TEnc: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.encoding.eq(&other.encoding)
    }
}

impl<TObj, TEnc> Eq for Sequence<TObj, TEnc>
where   TEnc: PartialEq {
}

impl<TObj, TEnc> Ord for Sequence<TObj, TEnc>
where   Sequence<TObj, TEnc>: Eq {
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

impl<TObj, TEnc> PartialOrd for Sequence<TObj, TEnc>
where   Sequence<TObj, TEnc>: Eq {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<TObj, TEnc> Sequence<TObj, TEnc>
where   TObj: Encodeable<Vec<TEnc>> + Fitness + Display + Default,
        TEnc: Clone + PartialEq,
        Vec<TEnc>: Decodeable<TObj> {

    pub fn create_by_object(obj: TObj) -> Self {
        let sequence = Sequence {
            phantom: PhantomData,
            encoding: obj.encode(),
            fitness: obj.get_fitness()
        };
        sequence
    }

    pub fn create_by_encoding(enc: Vec<TEnc>) -> Self {
        let obj = enc.decode();

        let sequence = Sequence {
            phantom: PhantomData,
            encoding: enc,
            fitness: obj.get_fitness()
        };
        sequence
    }

    pub fn create_random() -> Self {
        let obj = TObj::default();

        let sequence = Sequence {
            phantom: PhantomData,
            encoding: obj.encode(),
            fitness: obj.get_fitness()
        };
        sequence
    }

    pub fn crossover_random(parents: (&Self, &Self)) -> Self {
        assert!(parents.0.encoding.len() == parents.1.encoding.len());
        let split_point: usize = 1 + random(parents.0.encoding.len() as u32 - 2) as usize;

        let mut child_encoding = Vec::new();
        child_encoding.extend_from_slice(&parents.0.encoding[..split_point]);
        child_encoding.extend_from_slice(&parents.1.encoding[split_point..]);

        assert!(child_encoding.len() == parents.0.encoding.len());

        let obj = child_encoding.decode();

        let child = Sequence {
            phantom: PhantomData,
            encoding: child_encoding,
            fitness: obj.get_fitness()
        };

        child
    }

    pub fn get_encoding(&self) -> &Vec<TEnc> {
        &self.encoding
    }

    pub fn get_encoding_mut(&mut self) -> &mut Vec<TEnc> {
        &mut self.encoding
    }

    pub fn calculate_fitness(&mut self) {
        self.fitness = self.encoding.decode().get_fitness()
    }

    pub fn get_fitness(&self) -> i32 {
        self.fitness
    }





}
