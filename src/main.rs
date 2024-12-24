#[cfg(feature = "ssr")]
mod server_side;

#[cfg(feature = "ssr")]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    server_side::main()
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
