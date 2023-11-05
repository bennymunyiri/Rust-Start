// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     // Associated function to create a new Rectangle instance
//     fn new(width: u32, height: u32) -> Rectangle {
//         Rectangle { width, height }
//     }

//     // Method to calculate the area of the rectangle
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     // Creating a new Rectangle instance using the associated function
//     let rect = Rectangle::new(10, 20);
//     let rect1 = Rectangle::new(24, 64);

//     // Calling the method on the Rectangle instance
//     println!("Area of the rectangle: {}", rect1.area());
// }




//In this example, calculate_area is a method defined within the impl block for the Circle struct. It takes a reference to self and calculates the area of the circle based on its radius. Methods are called on instances of the struct using dot notation (circle.calculate_area()).
struct Circles {
    radius: f64,
}

impl Circle {
    fn calculate_area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

fn main() {
    let circle = Circles {
        radius: 3.0
    };
    let area = circle.calculate_area();
    println!("The area of the circle is: {}", area);
}

