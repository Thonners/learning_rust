use std::io;

fn main() {
    println!("Enter the temperature to be calculated, followed by F or C:");

    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let (temp, from_celcius) = split_input_into_temp_and_from_celcius(temp);

    if from_celcius {
        let temp_in_f = get_temp_in_farenheight(temp);
        println!("{temp}C = {temp_in_f}F");
    } else {
        let temp_in_c = get_temp_in_celcius(temp);
        println!("{temp}F = {temp_in_c}C");
    }
}

fn split_input_into_temp_and_from_celcius(input: String) -> (f64, bool) {
    if input.contains("F") {
        let temp: f64 = input
            .replace("F", "")
            .trim()
            .parse()
            .expect("Invalid number. You can only add a number followed by either F or C");
        println!("Farenheight. Value: {temp}");
        (temp, false)
    } else {
        println!("Converting from Celcius to F");
        let temp: f64 = input
            .replace("C", "")
            .trim()
            .parse()
            .expect("Invalid number. You can only add a number followed by either F or C");
        println!("Celcius. Value: {temp}");
        (temp, true)
    }
}

fn get_temp_in_celcius(temp: f64) -> f64 {
    (temp - 32.0) * 5.0 / 9.0
}

fn get_temp_in_farenheight(temp: f64) -> f64 {
    (temp * 9.0 / 5.0) + 32.0
}
