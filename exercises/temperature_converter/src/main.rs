use std::io;

#[derive(Debug, PartialEq, Eq)]
enum Unit {
    Celsius,
    Fahrenheit,
}

fn main() {
    let mut input_temp: String = String::new();
    let mut input_unit: String = String::new();

    println!("Enter the intput temperature unit: 'c' for Celsius or 'f' for Fahrenheit: ");
    io::stdin().read_line(&mut input_unit).unwrap();
    println!("Enter the temperature value: ");
    io::stdin().read_line(&mut input_temp).unwrap();

    let input_temp: f64 = match input_temp.trim().parse() {
        Ok(temp) => temp,
        Err(_) => panic!("Invalid temperature value"),
    };
    let input_unit: Unit = match input_unit.trim().parse() {
        Ok('f') => Unit::Fahrenheit,
        Ok('c') => Unit::Celsius,
        _ => panic!("Invalid input unit"),
    };

    let output_temp = convert_temperature(input_temp, input_unit);
    println!("{}", output_temp);
}

fn convert_temperature(input_temp: f64, input_unit: Unit) -> f64 {
    const THIRTY_TWO: f64 = 32.0;
    const NINE_FIFTHS: f64 = 9.0 / 5.0;
    assert!(input_unit == Unit::Celsius || input_unit == Unit::Fahrenheit);

    match input_unit {
        Unit::Celsius => (input_temp * NINE_FIFTHS) + THIRTY_TWO,
        Unit::Fahrenheit => (input_temp - THIRTY_TWO) / NINE_FIFTHS,
    }
}

#[test]
fn test_convert_temperature() {
    assert_eq!(convert_temperature(0.0, 'c'), 32.0);
    assert_eq!(convert_temperature(32.0, 'f'), 0.0);
    assert_eq!(convert_temperature(100.0, 'c'), 212.0);
    assert_eq!(convert_temperature(212.0, 'f'), 100.0);
}
