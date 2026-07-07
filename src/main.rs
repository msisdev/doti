use doti::fe::App;
use iocraft::prelude::*;

fn main() {
    smol::block_on(element!(App).render_loop()).unwrap();
}
