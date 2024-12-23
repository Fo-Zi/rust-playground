// Common interface for both structs
trait Printable {
    fn to_string(&self) -> String;
}

struct UserData {
    pub name: String,
    pub age: u8,
    pub height: f32,
}

// Concrete implementation of interface for UserData
impl Printable for UserData {
    fn to_string(&self) -> String {
        format!(
            "Name: {} - Age: {} - Height: {}",
            self.name, self.age, self.height
        )
    }
}

struct StoreItem {
    pub name: String,
    pub description: String,
    pub price: f32,
}

// Concrete implementation of interface for StoreItem
impl Printable for StoreItem {
    fn to_string(&self) -> String {
        format!(
            "Name: {} - Description: {} - Price: {}",
            self.name, self.description, self.price
        )
    }
}

fn print_dynamic(printable_obj: &dyn Printable) {
    println!("{}", printable_obj.to_string());
}

fn main() {
    // In this case, the objects are owned by each of the boxes, which belong to the vector
    let owned_printables: Vec<Box<dyn Printable>> = vec![
        Box::new(UserData {
            name: "Mario".into(),
            age: 32,
            height: 50_f32,
        }),
        Box::new(StoreItem {
            name: "Gamer PC".into(),
            description: "The best PC you'll ever find".into(),
            price: 50_000_f32,
        }),
    ];

    // We iterate over a reference now, since we don't want ownership of the boxes
    for printable in &owned_printables {
        print_dynamic(printable.as_ref());
    }
}
