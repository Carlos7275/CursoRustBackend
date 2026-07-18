use std::io;

fn main() {
    println!("Bienvenido al programa de entrada y salida en Rust");
    println!("===============================================");

    let mut nombre = String::new();
    println!("Ingresa tu nombre:");
    io::stdin()
        .read_line(&mut nombre)
        .expect("Error al capturar el nombre");

    let mut str_edad = String::new();
    println!("Ingresa tu edad:");
    io::stdin()
        .read_line(&mut str_edad)
        .expect("Error al capturar la edad");

    let edad: i32 = str_edad.trim().parse().expect("La edad debe ser un número");

    let mut str_estatura = String::new();
    println!("Ingresa tu edad:");
    io::stdin()
        .read_line(&mut str_estatura)
        .expect("Error al capturar la edad");

    let estatura: f32 = str_estatura
        .trim()
        .parse()
        .expect("La estatura debe ser un número flotante");

    println!("\nDatos capturados:");
    println!("Nombre: {}", nombre.trim());
    println!("Edad: {edad}");
    println!("Estatura: {estatura}");
}
