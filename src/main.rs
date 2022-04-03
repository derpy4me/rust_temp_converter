use std::io;

fn main() {
    println!("Temp Converter!");

    loop {
        println!("0: Quit");

        println!("1: Celsius to Fahrenheit");

        println!("2: Fahrenheit to Celsius");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line.");

        let choice: u8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if choice == 1 {
            celsius_input()
        } else if choice == 2 {
            fahrenheit_input()
        } else if choice == 0 {
            break;
        } else {
            continue;
        }
    }
}

fn celsius_input() {
    println!("\nEnter Celsius Temperature");

    let mut temp_cel = String::new();

    io::stdin()
        .read_line(&mut temp_cel)
        .expect("Failed to read line");

    let temp_cel: f32 = match temp_cel.trim().parse() {
        Ok(float) => float,
        Err(_) => 0.0,
    };

    let fahrenheit = celsius_convert_fahrenheit(temp_cel);

    fahrenheit_output(fahrenheit)
}

fn celsius_convert_fahrenheit(celsius: f32) -> f32 {
    //(0°C × 9/5) + 32 = 32°F
    {
        (celsius * 9.0 / 5.0) + 32.0
    }
}

fn fahrenheit_output(fahrenheit: f32) {
    println!("Converted Fahrenheit: {fahrenheit}\n")
}

fn fahrenheit_input() {
    println!("\nEnter Fahrenheit Temperature");

    let mut temp_fah = String::new();

    io::stdin()
        .read_line(&mut temp_fah)
        .expect("Failed to read line");

    let temp_fah: f32 = match temp_fah.trim().parse() {
        Ok(float) => float,
        Err(_) => 0.0,
    };

    let celsius = fahrenheit_convert_celsius(temp_fah);

    celsius_output(celsius)
}

fn fahrenheit_convert_celsius(fahrenheit: f32) -> f32 {
    // (32°F − 32) × 5/9 = 0°C
    {
        (fahrenheit - 32.0) * 5.0 / 9.0
    }
}

fn celsius_output(celsius: f32) {
    println!("Converted Celsius: {celsius}\n")
}
