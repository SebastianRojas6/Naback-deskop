use slint::{invoke_from_event_loop, Weak};
use crate::MainWrapper;

pub fn generate_enemies(main_weak: Weak<MainWrapper>) {
    std::thread::spawn(move || {
        loop {
            let weak = main_weak.clone();
            let _ = invoke_from_event_loop(move || {
                if let Some(main) = weak.upgrade() {

                    main.invoke_spawn_enemy();
                }
            });
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
    });
}
