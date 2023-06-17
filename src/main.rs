mod mix;

use mix::machine::Mix;


fn main() {

    let mut mix = Mix::new();

    mix.load_a(123);
    mix.store_a(0);

    mix.display_registers();
    mix.display_memory();

    println!("Hello, world!");
}
