/// This module contains functionality for Windows-specific assembly.
///
/// It provides utilities for assembling source code into machine code
/// compatible with the Windows operating system.
///
/// ```no_run
/// let shell_code = "echo hello!".to_string();
/// let executable = WindowsAssembler::assemble(shell_code).unwrap();
/// ```
pub mod windows;

/// This trait is designed to provide a common interface for different
/// types of assemblers. While currently focused on Windows-specific
/// assembly, it is designed with extensibility in mind, allowing
/// for potential support of other platforms such as Linux in the future.
pub trait Assembler {
    /// Assembles the provided source code into executable machine code.
    fn assemble(source: String) -> crate::result::Result<Vec<u8>>;
}
