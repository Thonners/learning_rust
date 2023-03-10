fn main() {
    println!("Hello, world!");

    another_function(55);
    print_labeled_measurement(5, 'h');
}

fn another_function(x: i32) {
    println!("Another function. {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
