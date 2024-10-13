use rust_embed::RustEmbed;

// The Ruby source code to be embedded
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, RustEmbed)]
#[folder = "src/embedded_ruby"]
pub struct Sources;

fn main() {
    println!("Hello, world from pure Rust!");
}
