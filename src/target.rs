use super::chromosome::Chromosome;

type Quality = u32;

pub struct TargetImage {
    pub pixels: Vec<u8>,
    pub height: u32,
    pub width: u32,
}

impl TargetImage {
    pub fn new(pixels: Vec<u8>, height: u32, width: u32) -> Self {
        TargetImage {
            pixels,
            height,
            width,
        }
    }

    pub fn fitness(&self, chromosome: &Chromosome) -> Quality {
        let mut base = 0;
        let diff = chromosome
            .0
            .iter()
            .enumerate()
            .map(|(idx, &gene)| {
                let u = u8::min(gene, self.pixels[idx]);
                let v = u8::max(gene, self.pixels[idx]);

                base += self.pixels[idx] as u32;
                (v - u) as u32
            })
            .sum::<u32>();

        base - diff
    }

    pub fn fitness_error(&self, chromosome: &Chromosome) -> f32 {
        let mse = {
            let n = chromosome.0.len() as f32;
            let sqrd_err = chromosome
                .0
                .iter()
                .enumerate()
                .map(|(idx, &gene)| {
                    let y = self.pixels[idx] as f32 / 255.0;
                    let y_hat = gene as f32 / 255.0;

                    (y - y_hat).powf(2.0)
                })
                .sum::<f32>();

            sqrd_err / n
        };

        1.0 - mse
    }
}
