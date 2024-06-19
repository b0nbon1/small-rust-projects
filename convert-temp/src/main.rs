// use std::io;
use clap::{CommandFactory, Parser};
use clap::error::ErrorKind;

enum TempUnit {
    C,
    F,
}

fn convert_temp(temp: f32, temptype: TempUnit) -> f32 {
    // (temp - 32.0) * 5.0 / 9.0
    match temptype {
        TempUnit::C => (temp * 9.0 / 5.0) + 32.0,
        TempUnit::F => (temp - 32.0) * 5.0 / 9.0,
    }
}

// fn main() {
//     let mut input_temp = String::new();
//     println!("Enter the temperature to convert: ");

//     io::stdin()
//         .read_line(&mut input_temp)
//         .expect("Failed to read line");

//     let input_temp: f32 = input_temp.trim().parse()
//         .expect("Invalid input");

    
//     let mut input_unit = String::new();

//     println!("Enter the unit of the temperature (C or F): ");

//     io::stdin()
//         .read_line(&mut input_unit)
//         .expect("Failed to read line");

//     let current_unit = match input_unit.trim()
//         .to_uppercase().as_str() {
//             "C" => TempUnit::C,
//             "F" => TempUnit::F,
//             _ => panic!("Invalid unit"),
//         };

//     let input_unit_str = match current_unit {
//         TempUnit::C => "Celsius",
//         TempUnit::F => "Fahrenheit",
//     };

//     // the temp_type variable will be owned by the convert function
//     let output_temp: f32 = convert_temp(input_temp, current_unit);


//     println!("Converted temp: {} degrees {}", output_temp, input_unit_str);
// }

#[derive(Parser)]
#[command(name = "convert-temp")]
#[command(version = "0.1")]
#[command(about = "Convert temperature values", long_about = None)]
struct Cli {
    /// The temperature value to convert, should be a number
    #[arg(long)]
    temp: String,

    /// The unit of the temperature value for Fahrenheit, if flag not provided, it will be assumed to be Celsius
    #[arg(short, long)]
    f: bool,
}

fn main() {
    let cli = Cli::parse();

    let input_temp: f32 = match cli.temp.trim().parse() {
        Ok(f32) => f32,
        Err(parse_float_error) => {
            println!("Please input a valid value. _{}_", parse_float_error);
            let mut cmd = Cli::command();
            cmd.error(
                ErrorKind::ValueValidation,
                "Temperature should be a float number.",
            )
            .exit();
        }
    };

    let convert_to: TempUnit = match cli.f {
            false => TempUnit::C,
            true => TempUnit::F,
        };

    let output_unit_str = match cli.f {
        true => "Celsius",
        false => "Fahrenheit",
    };
    let output_temp: f32 = convert_temp(input_temp, convert_to);

    println!("Converted temp: {} degrees {}", output_temp, output_unit_str);
}