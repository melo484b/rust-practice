// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// Refactor using tuples
// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// Would have to remember that dimensions.0 is width and dimensions.1 is height
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// Refactor with structs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Change area function into a method defined on the Rectangle struct
// impl denotes an implementation block
// Conveniently locates Rectangle's capabilities in one location
// Multiple impl blocks are valid syntax
impl Rectangle {
    // Method's first parameter is always self &self is used to borrow self immutably
    // Getter methods often have the same name as the field they return
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // Associated function - typically used for constructors
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // Associated functions are namespaced by the struct
    let sq = Rectangle::square(3);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!("Can rect1 hold rect2 {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3 {}", rect1.can_hold(&rect3));

    dbg!(&rect1);
}

// Function signature is very clear and easy to understand - moved to the Rectangle method
// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// Rust uses automatic referencing
// & &mut or * are added automatically so the object calling the method matches the method signature
// The following statements are equivalent
// p1.distance(&p2);
// (&p1).distance(&p2);