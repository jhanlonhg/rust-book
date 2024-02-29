use std::io;

fn main() {
    println!("Temperature Convertor");

    loop {
        println!("Please enter C or F for starting temperature scale.");

        let mut scale = String::new();

        io::stdin()
            .read_line(&mut scale)
            .expect("Failed to read line");

        let scale: TempScale = match scale.trim().to_uppercase().as_str() {
            "C" => TempScale::Celsius,
            "F" => TempScale::Fahrenheit,
            _ => continue,
        };

        println!("Please enter starting temperature.");

        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let result = convert_temperature(temp, &scale);

        let converted_scale_symbol = match scale {
            TempScale::Celsius => "F",
            TempScale::Fahrenheit => "C"
        };

        println!("The converted temperature is {result}°{converted_scale_symbol}");
        break
    }
}

enum TempScale {
    Celsius,
    Fahrenheit,
}

fn convert_temperature(temp: f64, scale: &TempScale) -> f64 {
    match scale {
        TempScale::Celsius => (temp * (9.0/5.0)) + 32.0,
        TempScale::Fahrenheit => (temp - 32.0) * (5.0/9.0)
    }
}