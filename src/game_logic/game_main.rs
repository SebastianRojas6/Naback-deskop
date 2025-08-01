slint::include_modules!();

pub fn game() -> Result<(), slint::PlatformError> {
    let game = Game::new()?;
    game.set_score(43);
    game.run()
}
