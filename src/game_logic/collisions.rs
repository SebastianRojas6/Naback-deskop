use crate::MainWrapper;
use slint::Weak;
use slint::invoke_from_event_loop;
use std::sync::{Arc, Mutex};

pub fn start_collision_checker(
    main_weak: Weak<MainWrapper>,
    enemy_positions: Arc<Mutex<Vec<f32>>>,
) {
    std::thread::spawn(move || {
        loop {
            let enemies = enemy_positions.lock().unwrap().clone();

            let w = main_weak.clone();
            invoke_from_event_loop(move || {
                if let Some(main) = w.upgrade() {
                    let px = main.get_main_char_x();
                    let py = main.get_main_char_y();
                    let pw = main.get_main_char_w();
                    let ph = main.get_main_char_h();

                    let mut collided = false;
                    for ex in &enemies {
                        let enemy_w = 60.0f32;
                        let enemy_h = 60.0f32;
                        let enemy_y = py; 

                        if px < ex + enemy_w
                            && px + pw > *ex
                            && py < enemy_y + enemy_h
                            && py + ph > enemy_y
                        {
                            collided = true;
                            break;
                        }
                    }
                    main.set_game_over(collided);
                }
            }).ok();

            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    });
}
