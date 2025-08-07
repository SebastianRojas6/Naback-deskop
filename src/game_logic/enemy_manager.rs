use crate::MainWrapper;
use slint::Weak;
use slint::invoke_from_event_loop;
use slint::{ModelRc, VecModel};
use std::sync::{Arc, Mutex};

pub fn start_enemy_manager(main_weak: Weak<MainWrapper>) -> Arc<Mutex<Vec<f32>>> {
    let positions = Arc::new(Mutex::new(Vec::<f32>::new()));

    {
        let positions_clone = positions.clone();
        let main_weak_clone = main_weak.clone();
        std::thread::spawn(move || {
            loop {
                {
                    let mut v = positions_clone.lock().unwrap();
                    v.iter_mut().for_each(|x| *x -= 3.0);
                    v.retain(|x| *x > -200.0);
                }

                let to_send = { positions_clone.lock().unwrap().clone() };

                let w = main_weak_clone.clone();
                let _ = invoke_from_event_loop(move || {
                    if let Some(main) = w.upgrade() {
                        let model = ModelRc::new(VecModel::from(to_send));
                        main.invoke_set_enemy_positions(model);
                    }
                });

                std::thread::sleep(std::time::Duration::from_millis(16));
            }
        });
    }

    {
        let positions_clone = positions.clone();
        std::thread::spawn(move || {
            loop {
                {
                    let mut v = positions_clone.lock().unwrap();
                    v.push(1400.0);
                }
                std::thread::sleep(std::time::Duration::from_secs(2));
            }
        });
    }
    positions
}
