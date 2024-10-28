// Tipos de Variables en Rust
fn main() {
    // 1. Variables inmutables (por defecto)
    let entero = 5;
    
    // 2. Variables mutables (usando mut)
    let mut numero = 10;
    numero = 15; // Esto es válido porque la variable es mutable
    
    // 3. Tipos numéricos enteros
    let entero_i32: i32 = 42;        // Entero con signo de 32 bits
    let entero_i64: i64 = 123456789; // Entero con signo de 64 bits
    let entero_u32: u32 = 100;       // Entero sin signo de 32 bits
    let entero_u8: u8 = 255;         // Entero sin signo de 8 bits
    
    // 4. Tipos de punto flotante
    let flotante_f32: f32 = 3.14;    // Flotante de 32 bits
    let flotante_f64: f64 = 2.71828; // Flotante de 64 bits
    
    // 5. Booleanos
    let verdadero: bool = true;
    let falso: bool = false;
    
    // 6. Caracteres
    let caracter: char = 'A';        // Usa comillas simples para caracteres
    
    // 7. Cadenas de texto
    let texto: &str = "Hola, mundo"; // String literal
    let string = String::from("Hola");// String dinámico
    
    // 8. Tuplas
    let tupla: (i32, f64, char) = (500, 6.4, 'Z');
    let (x, y, z) = tupla;           // Desestructuración
    
    // 9. Arrays
    let array: [i32; 5] = [1, 2, 3, 4, 5]; // Array de 5 elementos
    let primer_elemento = array[0];
    
    // 10. Constantes (deben tener tipo explícito)
    const PI: f64 = 3.141592653589793;
    
    // 11. Shadowing (redefinición de variables)
    let dato = "texto";
    let dato = dato.len(); // Ahora dato es un número
    
    // Imprimiendo algunos valores para demostración
    println!("Número mutable: {}", numero);
    println!("Entero i32: {}", entero_i32);
    println!("Flotante f64: {}", flotante_f64);
    println!("Booleano: {}", verdadero);
    println!("Caracter: {}", caracter);
    println!("String: {}", string);
    println!("Primer elemento del array: {}", primer_elemento);
    println!("Constante PI: {}", PI);
    println!("Dato después de shadowing: {}", dato);
}
