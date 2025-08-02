mod screens;
mod game_logic;
use screens::Pantallas;
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let pantalla = Pantallas::Main;
    pantalla.mostrar()
}
