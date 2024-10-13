use artichoke::prelude::*;
use rust_embed::RustEmbed;

// The Ruby source code to be embedded
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, RustEmbed)]
#[folder = "src/embedded_ruby"]
pub struct Sources;

/// Load Ruby sources into the Artichoke virtual file system.
///
/// # Errors
///
/// If an exception is raised on the Artichoke interpreter, it is returned.
pub fn init(interp: &mut Artichoke) -> Result<(), Error> {
    for source in Sources::iter() {
        if let Some(content) = Sources::get(&source) {
            interp.def_rb_source_file(&*source, content.data)?;
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world from pure Rust!");

    let mut interp = artichoke::interpreter()?;
    init(&mut interp)?;
    Ok(())
}
