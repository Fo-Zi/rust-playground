// In the same way as C, in Rust one can "group common data" into structs.
// These structs can have their implementations of interfaces, like the Animal interface
// below. And then our "derived" type, for example Bird, can call methods from the Animal interface

// In the end everything relates to data, so while Rust doesn't support inheritance, there
// are many ways of sharing functionality

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

// Animal interface implementation for Bird type. In this case it just uses the
// methods from the base type Animal
impl Animal for BirdData {
    fn eat(&self) {
        self.animal_data.eat();
    }

    fn sleep(&self) {
        self.animal_data.sleep();
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
