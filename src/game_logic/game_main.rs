use crate::MainWrapper;
use slint::{Weak, invoke_from_event_loop};

use super::generate_enemies::generate_enemies;
use super::timer::time;

pub fn game(main_weak: Weak<MainWrapper>) {
    {
        let weak_time = main_weak.clone();
        std::thread::spawn(move || {
            loop {
                let t = time();
                let weak = weak_time.clone();
                let _ = invoke_from_event_loop(move || {
                    if let Some(main) = weak.upgrade() {
                        main.set_time(t);
                    }
                });
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        });
    }

    {
        let weak_score = main_weak.clone();
        std::thread::spawn(move || {
            loop {
                let weak = weak_score.clone();
                let _ = invoke_from_event_loop(move || {
                    if let Some(main) = weak.upgrade() {
                        main.set_score(32);
                    }
                });
                std::thread::sleep(std::time::Duration::from_millis(300));
            }
        });
    }

    generate_enemies(main_weak.clone());
}
