use std::{thread, time};

fn main() {
    let now = time::Instant::now();
    let dur = time::Duration::from_millis(10000);
    thread::sleep(dur);
    println!("dur: {}.{:06}",
             now.elapsed().as_secs(),
             now.elapsed().subsec_nanos());
}
