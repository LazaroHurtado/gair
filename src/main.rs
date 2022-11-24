mod chromosome;
mod environment;
mod individual;
mod target;

use environment::Environment;

fn main() {
    let mut env = Environment::new(String::from("small.png"), 50, true);
    for i in 0..=30000 {
        env.step();
        if i % 1000 == 0 {
            println!("{:?}", env);
        } else if i % 500 == 0 {
            println!("i: {}", i);
        };
    }
    println!("Hello, world!");
}
