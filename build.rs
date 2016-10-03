#[cfg(not(feature = "nightly"))]
mod inner {
    extern crate serde_codegen;

    use std::env;
    use std::path::Path;

    pub fn main() {
        let out_dir = env::var_os("OUT_DIR").unwrap();

        let src = Path::new("src/schema/mod.rs");
        let dst = Path::new(&out_dir).join("schema.rs");

        serde_codegen::expand(&src, &dst).unwrap();
    }
}


#[cfg(feature = "nightly")]
mod inner {
    pub fn main() {}
}

fn main() {
    inner::main();
}
