use super::chromosome::{Chromosome, Crossover};
use super::individual::Individual;
use super::target::TargetImage;

use std::collections::BinaryHeap;
use std::f32;
use std::fmt::{Debug, Error as FmtError, Formatter, Result};

use image::{open, ImageFormat, RgbImage};

type Population = BinaryHeap<Individual>;

pub struct Environment {
    population: Population,
    grow_population: bool,
    target: TargetImage,
    capacity: usize,
    time: u32,
}

impl Environment {
    pub fn new(img_name: String, population_size: u16, grow_population: bool) -> Self {
        let img = open(img_name).unwrap().to_rgb8();
        let (height, width) = (img.height(), img.width());

        let pixels: Vec<u8> = img.into_raw();
        let pixels_len = pixels.len();

        let target = TargetImage::new(pixels, height, width);

        let initial_population = (0..population_size)
            .map(|_| {
                let makeup = Chromosome::new(pixels_len, 0.2);
                let fitness = target.fitness(&makeup);

                Individual::new(makeup, fitness, 0)
            })
            .collect::<Vec<Individual>>();

        Environment {
            population: BinaryHeap::from(initial_population),
            grow_population,
            target,
            capacity: population_size as usize,
            time: 0,
        }
    }

    fn increase_population(&mut self, rate: usize) {
        let mut next_gen_population: Population = BinaryHeap::new();

        (0..rate).step_by(3).for_each(|_| {
            let parent_a = self.population.pop().unwrap();
            let parent_b = self.population.pop().unwrap();
            let crossover = parent_a.makeup.crossover(&parent_b.makeup);

            next_gen_population.push(parent_a);
            next_gen_population.push(parent_b);

            let fitness = self.target.fitness(&crossover);
            let child = Individual::new(crossover, fitness, self.time);
            next_gen_population.push(child);
        });

        self.population = next_gen_population;
    }

    fn increase_capacity(&self) -> usize {
        let scaler = f32::consts::E.powf(self.time as f32 / 7000.0);
        scaler as usize
    }

    pub fn step(&mut self) {
        let mut next_capacity = self.capacity;
        if self.grow_population {
            next_capacity += self.increase_capacity()
        };

        self.time += 1;
        self.increase_population(next_capacity);
    }
}

impl Debug for Environment {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let elitism = self.population.peek().unwrap();
        let best_image = RgbImage::from_raw(
            self.target.width,
            self.target.height,
            elitism.makeup.0.clone(),
        )
        .ok_or(FmtError)?;

        let converted =
            best_image.save_with_format(format!("gen_{}.png", self.time), ImageFormat::Png);
        let Ok(_) = converted else {
            return Err(FmtError);
        };

        write!(
            f,
            "Saved best image from generation {}, with fitness of {} and current capacity is {}",
            self.time,
            self.target.fitness_error(&elitism.makeup),
            self.capacity
        )
    }
}
