use std::io;

fn main() {
    println!("Enter the unit you'd like to convert from.");
    println!("\'F\' for Fahrenheit, \'C\' for Celcius)");

    let unit: char = loop {
        let mut unit = String::new();

        io::stdin().read_line(&mut unit).expect("Failed to read line");

        let unit: char = match unit.trim().parse() {
            Ok(letter) => letter,
            Err(_) => continue,
        };

        if unit == 'C' || unit == 'c' 
        || unit == 'F' || unit == 'f' {
            break unit;
        }
    };
    
    println!("Enter the temperature value to convert.");

    let temp: f64 = loop {
        let mut temp = String::new();

        io::stdin().read_line(&mut temp).expect("Failed to read line");

        let _temp: f64 = match temp.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        };
    };

    let conv_temp: f64;

    if unit == 'f' || unit == 'F' {
        conv_temp = f_to_c(temp);
    } else {
        conv_temp = c_to_f(temp);
    }
    
    println!("{temp}{unit} -> {conv_temp}{}", 
            if unit == 'f' || unit == 'F' {'C'} else {'F'});
}


fn f_to_c(temp: f64) -> f64 {
    (temp - 32.0) * (5.0/9.0)
}

fn c_to_f(temp: f64) -> f64 {
    (temp * 5.0/9.0) + 32.0
}
