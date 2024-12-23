use delegate::delegate;

// Animal interface
pub trait Animal {
    fn eat(&self);
    fn sleep(&self);
}

// Animal base type
pub struct AnimalData {
    name: String,
    weight: f32,
}

// Animal interface implementation for a generic Animal type
impl Animal for AnimalData {
    fn eat(&self) {
        println!("{} is eating!", self.name);
    }

    fn sleep(&self) {
        println!("{} is sleeping.", self.name);
    }
}

// Bird interface
pub trait Bird {
    fn fly(&self);
    fn make_sound(&self);
}

// Bird base type
pub struct BirdData {
    animal_data: AnimalData,
    wing_width: f32,
    max_flying_height: f32,
}

// Using delegate, we don't need to do the boilerplate of forwarding calls, we just call the macro as below.
// It may seem unnecessary, but if you think on an interface that has many more methods, this just saves up time
impl Animal for BirdData {
    delegate! {
        to self.animal_data {
            fn eat(&self);
            fn sleep(&self);
        }
    }
}

// Bird interface implementation for Bird base type
impl Bird for BirdData {
    fn fly(&self) {
        println!("{} is flying!", self.animal_data.name);
    }

    fn make_sound(&self) {
        println!("{} is making sounds!", self.animal_data.name);
    }
}

fn main() {
    // Our concrete implementation of Bird
    let sparrow = BirdData {
        animal_data: AnimalData {
            name: "Sparrow".into(),
            weight: 0.3,
        },
        wing_width: 25.0,
        max_flying_height: 100.0,
    };

    // Inherited methods
    sparrow.eat();
    sparrow.sleep();

    // Bird specific methods
    sparrow.fly();
    sparrow.make_sound();
}
