use crate::MainWrapper;
use slint::ComponentHandle;
use crate::game_logic::game_main;

pub enum Pantallas {
    Main,
}

impl Pantallas {
    pub fn mostrar(&self) -> Result<(), slint::PlatformError> {
        match self {
            Pantallas::Main => {
                let main = MainWrapper::new()?;
                let weak = main.as_weak();

                main.on_navigate_to_game_with_instance(move || {
                    game_main::game(weak.clone());
                });

                main.run()
            }
        }
    }
}
