fn main() {
    let temp_fahrenheit: f64 = 0.0;
    let temp_celsius: f64 = -18.0;

    println!(
        "temp_fahrenheit converted: {}",
        convert_fahrenheit_to_celsius(temp_fahrenheit)
    );

    println!(
        "temp_celsius converted: {}",
        convert_celsius_to_fahrenheit(temp_celsius)
    )
}

fn convert_celsius_to_fahrenheit(temp_celsius: f64) -> f64 {
    return (temp_celsius * 1.8) + 32.0;
}

fn convert_fahrenheit_to_celsius(temp_fahrenheit: f64) -> f64 {
    return (temp_fahrenheit - 32.0) * 5.0 / 9.0;
}
