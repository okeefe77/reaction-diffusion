mod reaction;

use crate::reaction::Reaction;

#[allow(dead_code)]
struct Model {
    reaction: Reaction
}




fn main() {
    unsafe { std::env::set_var("RUST_BACKTRACE", "1"); }
    let mut r = Reaction::new(400, 300, 1.0, 0.5, 0.055, 0.062);

    println!("Before: {:?}", r.sample_cell(199, 150));

    r.seed(200, 150);
    r.step();
    
    println!("After: {:?}", r.sample_cell(199, 150));
}
