use std::io;
use rand::{RngExt};

fn main() {
    println!("Demo de condicionales con ciclos, funciones y randoms");
    println!("========================================================");

    println!("Ingrese un numero:");

    let mut str_num: String = String::new();

    let mut rng = rand::rng();
    let numero_random = rng.random_range(1..=100);

    io::stdin()
        .read_line(&mut str_num)
        .expect("Error al capturar el numero");

    let num: i64 = str_num.trim().parse().expect("El numero debe ser numerico");

    println!(
        "El numero {num} es {}",
        if es_par(num) { "par" } else { "impar" }
    );

    println!(
        "El factorial del numero {num} es: {}",
        calcular_factorial(num)
    );

    println!(
        "La potencia del numero {num} ala {numero_random} es : {}",
        potencia(num as f64, numero_random)
    );
}

fn calcular_factorial(num: i64) -> i64 {
    let mut factorial = 1;

    for iterador in 1..num + 1 {
        factorial *= iterador;
    }

    factorial
}

fn es_par(num: i64) -> bool {
    num % 2 == 0
}

fn potencia(num: f64, potencia: i64) -> f64 {
    let mut resultado: f64 = 1.0;
    let mut iterador: i64 = 0;
    while iterador < potencia {
        resultado *= num;
        iterador += 1;
    }

    return if potencia < 0 {
        1.0 / resultado
    } else {
        resultado
    };
}
