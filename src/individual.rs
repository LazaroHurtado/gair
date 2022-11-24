use super::chromosome::Chromosome;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter, Result};

#[derive(Clone, Eq)]
pub struct Individual {
    pub makeup: Chromosome,
    pub fitness: u32,
    pub generation: u32,
}

impl Individual {
    pub fn new(makeup: Chromosome, fitness: u32, generation: u32) -> Self {
        Individual {
            makeup,
            fitness,
            generation,
        }
    }
}

impl Ord for Individual {
    fn cmp(&self, other: &Self) -> Ordering {
        self.fitness.cmp(&other.fitness)
    }
}

impl PartialOrd for Individual {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Individual {
    fn eq(&self, other: &Self) -> bool {
        self.fitness == other.fitness
    }
}

impl Debug for Individual {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "G-{}, F-{}", self.generation, self.fitness)
    }
}
