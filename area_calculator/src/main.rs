struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rect is {} square pixels.",
        area_individual(width1, height1)
    );

    let rect1 = (30, 50);
    println!(
        "The area of the rect from a tuple is {} square pixels.",
        area_tuple(rect1)
    );

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rect from a struct is {} square pixels.",
        area_struct(rect2)
    );
}

fn area_individual(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rect: Rectangle) -> u32 {
    rect.width * rect.height
}