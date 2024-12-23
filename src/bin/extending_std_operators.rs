use std::ops::{Add,Sub};

pub struct Point{
    x: f32,
    y: f32,
    z: f32
}

// For this test, the traits are implemented for Point references, since it's more efficient to have to be copying data around
impl Add for &Point {
    type Output = Point;

    fn add(self, other: Self) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

// For this test, the traits are implemented for Point references, since it's more efficient to have to be copying data around
impl Sub for &Point {
    type Output = Point;

    fn sub(self, other: Self) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

fn main() {

    let p1 = Point { x: 1.0, y: 2.0 , z: 3.0 };
    let p2 = Point { x: 3.0, y: 4.0 , z: 5.0 };
    
    // Adding two points
    let p3 = &p1 + &p2;
    assert_eq!(p3.x , 4.0);
    assert_eq!(p3.y , 6.0);
    assert_eq!(p3.z , 8.0);

    // Substracting two points
    let p4 = &p1 - &p2;
    assert_eq!(p4.x , -2.0);
    assert_eq!(p4.y , -2.0);
    assert_eq!(p4.z , -2.0);

}