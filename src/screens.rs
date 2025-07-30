slint::include_modules!();

pub enum Pantallas {
    Menu,
    Fotos,
    Cartas,
    Juego,
}

impl Pantallas {
    pub fn mostrar(&self) -> Result<(), slint::PlatformError> {
        match self {
            Pantallas::Menu => Menu::new()?.run(),
            Pantallas::Fotos => Photos::new()?.run(),
            Pantallas::Cartas => Cards::new()?.run(),
            Pantallas::Juego => Game::new()?.run(),
        }
    }
}

pub fn eleccion(x: i32) -> Pantallas {
    match x {
        0 => Pantallas::Menu,
        1 => Pantallas::Fotos,
        2 => Pantallas::Cartas,
        3 => Pantallas::Juego,
        _ => {
            println!("Opción inválida");
            Pantallas::Menu
        }
    }
}
