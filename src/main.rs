slint::include_modules!();

mod screens;
mod game_logic;
use screens::eleccion;

fn main() -> Result<(), slint::PlatformError> {

    loop {
        println!("0: Menú, 1: Fotos, 2: Cartas, 3: Juego, 4: Salir");

        let mut x = String::new();
        std::io::stdin().read_line(&mut x).expect("Error");

        let x_int: i32 = match x.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Por favor, ingresa un número válido.");
                continue;
            }
        };

        if x_int == 4 {
            break;
        }

        let pantalla = eleccion(x_int);
        pantalla.mostrar().expect("Error");
    }

    Ok(())
}
