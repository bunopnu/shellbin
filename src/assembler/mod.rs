/// This module contains Windows-specific functionalities.
pub mod windows;

/// This module contains Linux-specific functionalities.
pub mod linux;

/// This trait is designed to provide a common interface for different
/// types of assemblers.
pub trait Assembler {
    /// Assembles the provided source code into executable machine code.
    fn assemble(source: String) -> crate::result::Result<Vec<u8>>;
}
