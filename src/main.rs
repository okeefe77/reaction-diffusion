struct RDCell {
    a: f32,
    b: f32
}

struct RDGrid {
    data: Vec<RDCell>,
    width: u32,
    height: u32
}

struct Reaction {
    grid: RDGrid,
    a_rate: f32,
    b_rate: f32,
    feed: f32,
    kill: f32
}

struct Model {
    reaction: Reaction
}

fn main() {
    println!("Hello, world!");
}
