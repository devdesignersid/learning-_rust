#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    let scale = 2;

    let scaled_rectangle = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    let scaled_rectangle_area = area(&scaled_rectangle);

    println!("Scaled rectangle area: {}", scaled_rectangle_area);

    println!("{:?}", rectangle); // Prints {width: 30, height: 50}
     //
     //  Prints:
     //    Rectangle {
     //     width: 30,
     //     height: 50,
     // }
     //
    println!("{:#?}", rectangle);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rectangle)
    );

    // Debug macro expression
    dbg!(&rectangle);

}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
