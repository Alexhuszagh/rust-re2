fn main() {
    let dst = cmake::build("re2");


    cc::Build::new()
        .cpp(true)
        .file("match.cc")
        .include(dst.join("include"))
        .compile("libmatch.a");

    println!("cargo:rerun-if-changed=src/build.rs");
    println!("cargo:rerun-if-changed=match.cc");
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=re2");
}
