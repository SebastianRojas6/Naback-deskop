slint::include_modules!();

mod screens;
use screens::eleccion;

fn main() -> Result<(), slint::PlatformError> {

    println!("0: Menú, 1: Fotos, 2: Cartas, 3: Juego");

    let mut x = String::new();
    std::io::stdin().read_line(&mut x).expect("Error");
    let x_int: i32 = x.trim().parse().expect("Error");

    let pantalla = eleccion(x_int);
    pantalla.mostrar()
}
