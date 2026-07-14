use wasm_bindgen::prelude::*;

// Esta función auxiliar determina si un número es primo
// Usamos tipos de datos fijos (u32) para aprovechar al máximo el hardware
#[wasm_bindgen]
pub fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    
    // Solo buscamos divisores hasta la raíz cuadrada del número
    let limit = (n as f64).sqrt() as u32;
    let mut i = 3;
    while i <= limit {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

// Esta es la función principal que va a llamar JavaScript
// Cuenta cuántos números primos hay desde 2 hasta el límite dado
#[wasm_bindgen]
pub fn count_primes(limit: u32) -> u32 {
    let mut count = 0;
    let mut i = 2;
    while i < limit {
        if is_prime(i) == true {
            count += 1;
        }
        i += 1;
    }
    count
}