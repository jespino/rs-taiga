#[cfg(not(feature = "serde_macros"))]
mod inner {
    extern crate syntex;
    extern crate serde_codegen;

    use std::env;
    use std::path::Path;

    pub fn main() {
        let mut registry = syntex::Registry::new();
        serde_codegen::register(&mut registry);

        registry.expand(
            "taiga",
            &Path::new("src/lib.in.rs"),
            &Path::new(&env::var_os("OUT_DIR").unwrap()).join("lib.rs")
        ).unwrap();
    }
}

#[cfg(feature = "serde_macros")]
mod inner {
    pub fn main() {}
}

fn main() {
    inner::main();
}
