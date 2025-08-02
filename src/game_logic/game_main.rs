use super::timer::time;
use slint::{invoke_from_event_loop, ComponentHandle};
use crate::Game; 

pub fn game() -> Result<(), slint::PlatformError> {

    let game: Game = Game::new()?;

    let game_weak_for_time = game.as_weak();
    let game_weak_for_score = game.as_weak();

    std::thread::spawn(move || {
        loop {
            let t = time();
            let game_weak = game_weak_for_time.clone();
            let _ = invoke_from_event_loop(move || {
                if let Some(game) = game_weak.upgrade() {
                    game.set_time(t);
                }
            });
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    std::thread::spawn(move || {
        loop {
            let game_weak = game_weak_for_score.clone();
            let _ = invoke_from_event_loop(move || {
                if let Some(game) = game_weak.upgrade() {
                    game.set_score(32);
                }
            });
            std::thread::sleep(std::time::Duration::from_millis(300));
        }
    });

    game.run()
}