fn main() {
    let target = std::env::var("TARGET").unwrap();

    if target != "aarch64-unknown-linux-gnu" {
        panic!("This crate can only be built for the target aarch64-unknown-linux-gnu");
    }
}
