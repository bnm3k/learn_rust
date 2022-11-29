#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use std::fmt::Debug;

trait Animal {
    fn new(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

struct Sheep {
    naked: bool,
    name: &'static str,
}

impl Animal for Sheep {
    fn new(name: &'static str) -> Sheep {
        Sheep {
            naked: false,
            name: name,
        }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn noise(&self) -> &'static str {
        "baaah"
    }
}

struct Donkey {}

trait NoisyAnimal {
    fn noise(&self) -> &'static str;
}

impl NoisyAnimal for Donkey {
    fn noise(&self) -> &'static str {
        "neight"
    }
}
fn get_noise_animal() -> Box<dyn NoisyAnimal> {
    Box::new(Donkey {})
}

fn get_animal(name: &'static str) -> impl Animal {
    let s = Sheep::new(name);
    return s;
}

fn use_animal(a: impl Animal) {
    println!("{} says {}", a.name(), a.noise());
}

fn main() {
    let s = Sheep::new("dolly");
    use_animal(get_animal("doll"));
}
