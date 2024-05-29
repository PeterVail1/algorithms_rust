use gs::driver;
use dp::dynamicprogramming;
mod dp {
    pub mod dynamicprogramming;
}
mod gs {
    pub mod driver;
}
fn main() { 
    driver::main();
    dynamicprogramming::runner();
}    