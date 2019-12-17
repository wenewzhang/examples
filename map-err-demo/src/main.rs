fn main() {
    foo()
    .map_err(|err| println!("{:?}", err))
    .ok();
}

fn foo() -> Result<(), std::io::Error> {
    Ok(())
}
