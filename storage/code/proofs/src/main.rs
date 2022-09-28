// Consider the following definitions:
trait Animal {
    fn make_sound(&self);
}

struct Dog;

impl Animal for Dog {
    fn make_sound(&self) { println!("Ruff"); }
}

struct Cat;

impl Animal for Cat {
    fn make_sound(&self) { println!("Meow"); }
}

struct Chinchilla;

impl Animal for Chinchilla {
    fn make_sound(&self) { println!("..."); }
}

struct AnimalContainer{
    animals: Vec::<Box<dyn Animal>>
}

impl AnimalContainer{
    pub fn new() -> Self {
        let mut animals = Vec::<Box<dyn Animal>>::new();
        animals.push(Box::new(Dog));
        animals.push(Box::new(Cat));
        animals.push(Box::new(Chinchilla));
        return   AnimalContainer {
            animals
        };
    }
}


fn main() {
    // Accompanied with the following code:
    let animal_container = AnimalContainer::new();

    for animal in animal_container.animals {
        animal.make_sound();
    }
}
