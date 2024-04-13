/// This module contains Windows-specific functionalities.
pub mod windows;

/// This module contains Linux-specific functionalities.
pub mod linux;

/// Represents the size of the data segment.
pub(super) const ABSOLUTE_MAXIMUM: usize = 8192;

/// Represents the maximum length of a shell command.
pub(super) const PREFFERED_MAXIMUM: usize = 8148;

/// This trait is designed to provide a common interface for different
/// types of assemblers.
pub trait Assembler {
    /// Assembles the provided source code into executable machine code.
    fn assemble(source: String) -> crate::result::Result<Vec<u8>>;
}
