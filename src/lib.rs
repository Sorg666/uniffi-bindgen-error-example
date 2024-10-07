uniffi::include_scaffolding!("lib");

pub struct Something {
    pub name: String,
}

pub fn return_something() -> Something {
    Something { name: String::from("Hello") }
}