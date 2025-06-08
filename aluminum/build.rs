fn main() {
    println!("cargo:rustc-link-search=native=/opt/homebrew/opt/openblas/lib");
    println!("cargo:rustc-link-lib=openblas");

    // Force the linker to look in the right place
    println!("cargo:rustc-link-arg=-L/opt/homebrew/opt/openblas/lib");
    println!("cargo:rustc-link-arg=-lopenblas");

    // Make sure the runtime can find the library
    println!("cargo:rustc-link-arg=-Wl,-rpath,/opt/homebrew/opt/openblas/lib");
}
