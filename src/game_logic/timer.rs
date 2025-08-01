use std::time::Instant;
use once_cell::sync::Lazy;

static START_TIME: Lazy<Instant> = Lazy::new(Instant::now);

pub fn time() -> i32 {
    let elapsed = START_TIME.elapsed().as_secs();
    elapsed as i32
}
