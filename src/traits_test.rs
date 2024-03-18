pub fn run_trait_test() {
    let d1: &dyn AnimalSound = &Dog{};
    let a1: &dyn AnimalSound = &Antelope{};

    make_some_noise(d1);
    make_some_noise(a1);

    let d2: &dyn AnimalEating = &Dog{};
    let a2: &dyn AnimalEating = &Antelope{};

    make_some_food(d2);
    make_some_food(a2);

    let b1 = get_animal();
    b1.eat_food();
    b1.make_sound();
}

// super traits
fn get_animal() -> Box<dyn Animal> {
    let animal = Dog{};

    return Box::from(animal);
}

// returning dyn trait
// fn get_animal() -> Box<dyn AnimalEating> {
//     let animal = Bear{};

//     return Box::from(animal);
// }

// fn get_animal() -> Box<dyn AnimalSound> {
//     let bear = Bear{};

//     return Box::from(bear);
// }

// fn make_some_noise<Animal: AnimalSound>(a: Animal) {
//     a.make_sound();
// }

fn make_some_noise(a: &dyn AnimalSound) {
    a.make_sound();
}

fn make_some_food(a: &dyn AnimalEating) {
    a.eat_food();
}

struct Dog {}

struct Antelope {}

struct Bear{}

trait Animal: AnimalEating + AnimalSound {}

trait AnimalEating {
    fn eat_food(&self);
}

trait AnimalSound {
    fn make_sound(&self);
}

impl AnimalEating for Dog {
    fn eat_food(&self) {
        println!("Dog is eating dog food");
    }
}

impl AnimalSound for Dog {
    fn make_sound(&self) {
        println!("Dog is barking");
    }
}

impl Animal for Dog {}

impl AnimalEating for Antelope {
    fn eat_food(&self) {
        println!("Antilope is eating natural desert plants");
    }
}

impl AnimalSound for Antelope {
    fn make_sound(&self) {
        println!("Antelope is bleating");
    }
}

impl Animal for Antelope {}

impl AnimalEating for Bear {
    fn eat_food(&self) {
        println!("Bear is eating some other animal");
    }
}

impl AnimalSound for Bear {
    fn make_sound(&self) {
        println!("Bear is roaring");
    }
}

impl Animal for Bear {}
