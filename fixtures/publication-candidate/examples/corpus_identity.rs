//! Print the BLAKE3 identity of the shared corpus source.

fn main() {
    println!(
        "{}",
        blake3::hash(include_bytes!("../src/corpus.rs")).to_hex()
    );
}
