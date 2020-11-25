mod arnie;
#[macro_use]
extern crate stdweb;
use stdweb::js_export;


#[js_export]
pub fn get_arnie(phrase: String) -> String {
    arnie::get_arnie(phrase, 1u64)
}

fn main() {
    stdweb::initialize();
}