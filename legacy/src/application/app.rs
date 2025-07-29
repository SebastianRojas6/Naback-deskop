use std::{cell::Cell, rc::Rc};

pub enum AppState{
    Menu,
    //Game,
    //Cards,
    //Photos,
}

pub struct MyApp {
    _value: Rc<Cell<u32>>,
    _spin: bool,
    _blinky: bool,
    state : AppState,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            _value: Rc::new(Cell::new(42)),
            _spin: false,
            _blinky: false,
            state: AppState::Menu,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
            
            match self.state {

                AppState::Menu => self.mostrar_menu(ctx),   

            }

    }

}