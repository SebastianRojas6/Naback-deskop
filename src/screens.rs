use crate::MainWrapper;
use crate::game_logic::game_main;
use slint::ComponentHandle;

pub enum Pantallas {
    Main,
    Juego,
}

impl Pantallas {
    pub fn mostrar(&self) -> Result<(), slint::PlatformError> {
        match self {
            Pantallas::Main => {
                let main = MainWrapper::new()?;
                let weak = main.as_weak();

                main.on_navigate_to_game(move || {
                    if let Some(_) = weak.upgrade() {
                        let _ = Pantallas::Juego.mostrar();
                    }
                });

                main.run()
            }

            Pantallas::Juego => game_main::game(),
        }
    }
}
