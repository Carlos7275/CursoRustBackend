mod calculator;

use std::io;

use calculator::calculator::Calculator;

fn main() {
    println!("Modulos , Estructuras y IMPL!");
    println!("============================================");

    let mut str_num1: String = String::new();
    let mut str_num2: String = String::new();

    println!("Ingrese un numero:");
    io::stdin()
        .read_line(&mut str_num1)
        .expect("Error al capturar el numero 1");

    println!("Ingrese otro numero:");
    io::stdin()
        .read_line(&mut str_num2)
        .expect("Error al capturar el numero 2");

    let num1: f64 = str_num1.trim().parse().expect("Numero 1 debe ser numerico");
    let num2: f64 = str_num2.trim().parse().expect("Numero 2 debe ser numerico");

    let calculator: Calculator = Calculator::new(num1, num2);

    println!(
        "{num1} + {num2} = {} {num1} - {num2} = {} {num1} * {num2} = {} {num1} / {num2} = {}",
        calculator.add(),
        calculator.sub(),
        calculator.mul(),
        calculator.div(),
    )
}
