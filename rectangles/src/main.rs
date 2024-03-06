fn main() {
    let width1 = 30;
    let height1 = 60;

    println!(
        "The area of the naive rectangle is {} units",
        naive_area(width1, height1)
    );

    let rect1 = (40, 50);

    println!(
        "The area of the tuple rectangle is {} units",
        tuple_area(rect1)
    );

    let rect2 = Rectangle { width: 45, height: 45};

    println!(
        "The area of the struct rectangle is {} units",
        struct_area(&rect2)
    );

    println!(
        "The area of the rectangle using a method is {} units",
        rect2.area()
    );

    println!(
        "rect2's debug output is {:?}",
        rect2
    );

    println!(
        "rect2 pretty debug is {:#?}",
        rect2
    );

    let rect3 = Rectangle { width: 25, height: 30 };
    let rect4 = Rectangle { width: 50, height: 40 };

    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));
    println!("Can rect2 hold rect4? {}", rect2.can_hold(&rect4));

    let _square = Rectangle::square(25);

}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn naive_area(width: u32, height: u32) -> u32 {
    width * height
}

fn tuple_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn struct_area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}