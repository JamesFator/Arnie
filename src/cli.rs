mod arnie;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards").as_millis() as u64;
    println!("{}", arnie::get_arnie(args.join(" "), seed));
}
