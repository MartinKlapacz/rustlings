// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` for hints!

fn main() {
    let cat: (&str, f64) = ("Furry McFurson", 3.5);
    let (name, age): (&str, f64) = cat;

    println!("{} is {} years old.", name, age);
}
