fn main() {
    println!("cargo:rustc-link-search=native=./c_lib");
    println!("cargo:rustc-link-lib={}", "demo");
}
