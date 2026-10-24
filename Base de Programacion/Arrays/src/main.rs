use std::io;
use rand::Rng;

fn main() {
    println!("Manejo de arrays en RUST");
    println!("=================================");

    println!("¿Cuántos números deseas generar?");

    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).unwrap();

    let cantidad: usize = entrada.trim().parse().expect("Cantidad inválida");

    let limite = cantidad.max(100) as i64;

    let mut numeros: Vec<i64> = (0..cantidad)
        .map(|_| rand::thread_rng().gen_range(1..=limite))
        .collect();

    let sumatoria: i64 = numeros.iter().sum();
    let min = numeros.iter().min().unwrap();
    let max = numeros.iter().max().unwrap();

    println!("Arreglo generado: {:?}", numeros);
    println!("Ingresa un número a buscar:");

    let num_a_buscar: i64 = capturar_input().trim().parse().expect("Número inválido");
    let posicion =buscar_elemento_array(num_a_buscar,&numeros);

    println!(
        "{}",
        if posicion != -1 {
            format!("El número {num_a_buscar} está en la posición {posicion}")
        } else {
            format!("El número {num_a_buscar} no se encontró")
        }
    );
    println!("Sumatoria: {sumatoria}, Mínimo: {min}, Máximo: {max}");

    println!("Ordenando array....");
    numeros.sort();

    println!("Eliminando datos duplicados...");
    numeros.dedup();

    println!("Arreglo ordenado y sin dátos duplicados: {:?}", numeros);

}
/// Busca un número en el slice y devuelve su posición.
/// Retorna -1 si no lo encuentra.
fn buscar_elemento_array(num_a_buscar: i64, numeros: &[i64]) -> i64 {
    numeros.iter()
        .position(|&n| n == num_a_buscar)
        .map(|i| i as i64)
        .unwrap_or(-1)
}

/// Captura una entrada del teclado y elimina espacios externos.
fn capturar_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}