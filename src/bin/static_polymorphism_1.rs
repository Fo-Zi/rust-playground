// Common interface for both structs
trait Printable {
    fn to_string(&self) -> String;
}

struct UserData{
    pub name: String,
    pub age: u8,
    pub height: f32
}

// Concrete implementation of interface for UserData
impl Printable for UserData {
    fn to_string(&self) -> String {
        format!("Name: {} - Age: {} - Height: {}",self.name,self.age,self.height)
    }
}

struct StoreItem{
    pub name: String,
    pub description: String,
    pub price: f32
}

// Concrete implementation of interface for StoreItem
impl Printable for StoreItem {
    fn to_string(&self) -> String {
        format!("Name: {} - Description: {} - Price: {}",self.name,self.description,self.price)
    }
}

fn print_static<T: Printable>(printable_obj: &T) {
    println!("{}",printable_obj.to_string());
}

fn main(){

    let my_user = UserData {
        name: "Mario".into(),
        age: 32,
        height: 50_f32
    };
    
    let my_item = StoreItem {
        name: "Gamer PC".into(),
        description: "The best PC you'll ever find".into(),
        price: 50_000_f32 
    };

    print_static(&my_user); 
    print_static(&my_item); 
    
}