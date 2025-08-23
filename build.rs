fn main() {
    setup_esp();
}

#[cfg(feature = "esp-build")]
fn setup_esp() {
    embuild::espidf::sysenv::output();
}

#[cfg(not(feature = "esp-build"))]
fn setup_esp() {
    // Do nothing
}