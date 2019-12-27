fn some_fn() -> ! {
    panic!("hellll o!")
}

// fn some_fn() {
//     ()
// }
fn main() {
    let _a: () = some_fn();
    println!("This functions returns and you can see this line.")
}
