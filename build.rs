fn main() {
    cc::Build::new()
        .file("src/foo.c")
        .compile("libfooc.a");

    nasm_rs::compile_library("libfooasm.a", &["src/foo.s"])
        .unwrap();
}
