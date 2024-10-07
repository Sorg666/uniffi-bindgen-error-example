#[cfg(feature = "cli")]
fn main() {
    uniffi::uniffi_bindgen_main()
}

#[cfg(not(feature = "cli"))]
fn main() {
    // nop
}