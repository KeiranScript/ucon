use ucon::{Length, Temperature, Time, Volume, Weight};

fn main() {
    // Length conversions
    let meter = Length::Meter(1.0);
    let kilometer = meter.convert_to(&Length::Kilometer(0.0)).unwrap();
    println!("{} is {}", meter, kilometer);

    // Weight conversions
    let kilogram = Weight::Kilogram(1.0);
    let gram = kilogram.convert_to(&Weight::Gram(0.0)).unwrap();
    println!("{} is {}", kilogram, gram);

    // Temperature conversions
    let celsius = Temperature::Celsius(25.0);
    let fahrenheit = celsius.convert_to(&Temperature::Fahrenheit(0.0)).unwrap();
    println!("{} is {}", celsius, fahrenheit);

    // Volume conversions
    let liter = Volume::Liter(1.0);
    let milliliter = liter.convert_to(&Volume::Milliliter(0.0)).unwrap();
    println!("{} is {}", liter, milliliter);

    // Time conversions
    let second = Time::Second(3600.0);
    let hour = second.convert_to(&Time::Hour(0.0)).unwrap();
    println!("{} is {}", second, hour);
}
