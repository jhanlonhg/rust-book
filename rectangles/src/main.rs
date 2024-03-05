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
        "rect2's debug output is {:?}",
        rect2
    );

    println!(
        "rect2 pretty debug is {:#?}",
        rect2
    );

}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
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