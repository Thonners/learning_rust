#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other_rect: &Self) -> bool {
        self.width > other_rect.width && self.height > other_rect.height
    }
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
        area_struct(&rect2)
    );
    println!(
        "The area of the rect from a struct method is {} square pixels.",
        rect2.area()
    );
    println!("Rect is {:#?}", rect2);

    let big_rect = Rectangle {
        width: 35,
        height: 45,
    };
    let small_rect = Rectangle {
        width: 30,
        height: 10,
    };
    println!(
        "Big rect can hold small rect: {}",
        big_rect.can_hold(&small_rect)
    );
    println!("Big rect can hold rect2: {}", big_rect.can_hold(&rect2));
}

fn area_individual(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
