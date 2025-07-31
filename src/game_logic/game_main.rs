slint::include_modules!();

pub fn game() -> Result<(), slint::PlatformError> {

    let game = Game::new()?;
    game.set_bro(43);
    game.run()

    
}