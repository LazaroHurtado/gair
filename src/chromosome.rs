use rand::Rng;

pub type Gene = u8;
pub type MutationRate = f32;

pub trait Crossover {
    fn crossover(&self, partner: &Self) -> Self;
}

trait Mutation {
    fn mutate(&mut self);
}

#[derive(Clone)]
pub struct Chromosome(pub Vec<Gene>, pub MutationRate);

impl Chromosome {
    pub fn new(length: usize, mutation_rate: MutationRate) -> Self {
        let chromosome = (0..length)
            .map(|_| {
                let mut rng = rand::thread_rng();
                rng.gen_range(0..=255)
            })
            .collect::<Vec<Gene>>();
        let mut individual = Chromosome(chromosome, mutation_rate);
        individual.mutate();

        individual
    }

    fn pixelwise_crossover(&self, other: &Self) -> Vec<Gene> {
        let mut rng = rand::thread_rng();
        self.0
            .iter()
            .enumerate()
            .map(|(idx, &gene)| {
                let p = rng.gen_range(0.0..1.0);

                if p >= 0.5 {
                    return gene;
                };
                other.0[idx]
            })
            .collect::<Vec<Gene>>()
    }

    fn ranged_crossover(&self, other: &Self) -> Vec<Gene> {
        let range: usize = 20;

        self.0
            .iter()
            .enumerate()
            .map(|(idx, &gene)| {
                if (idx / range) % 2 == 0 {
                    return gene;
                };
                other.0[idx]
            })
            .collect::<Vec<Gene>>()
    }

    fn singlepoint_crossover(&self, other: &Self) -> Vec<Gene> {
        let mut rng = rand::thread_rng();

        let n = self.0.len() as f32;
        let crossover_point = rng.gen_range(n * 0.45..=n * 0.65) as usize;

        let mut crossover: Vec<Gene> = Vec::new();
        let (mut parent_a, mut parent_b) = (self.0.iter(), other.0.iter());
        for gene in 0..n as usize {
            if gene <= crossover_point {
                crossover.push(*parent_a.next().unwrap())
            } else {
                crossover.push(*parent_b.next().unwrap());
            };
        }

        crossover
    }
}

impl Eq for Chromosome {}

impl PartialEq for Chromosome {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Mutation for Chromosome {
    fn mutate(&mut self) {
        let mut rng = rand::thread_rng();
        let p = rng.gen_range(5..(self.0.len() as f32 * 0.20) as usize);

        (0..p).for_each(|_| {
            let q = rng.gen_range(0.0..1.0);
            if q <= self.1 {
                let n = rng.gen_range(0..self.0.len());
                self.0[n] = rng.gen_range(0..=255);
            };
        });
    }
}

impl Crossover for Chromosome {
    fn crossover(&self, partner: &Chromosome) -> Chromosome {
        let mut rng = rand::thread_rng();

        let crossover = match rng.gen_range(0.0..1.0) {
            x if x >= 0.33 => self.pixelwise_crossover(partner),
            x if x <= 0.66 => self.ranged_crossover(partner),
            _ => self.singlepoint_crossover(partner),
        };

        Chromosome(crossover, self.1 * 0.99995)
    }
}
