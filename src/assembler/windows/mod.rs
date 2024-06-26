use crate::assembler::Assembler;

/// Represents the size of generated binary.
const BINARY_SIZE: usize = 16384;

/// Represents the executable data.
const EXECUTABLE: &[u8; 1536] = include_bytes!("../../../tbin/windows.s");

/// A struct representing a Windows-specific assembler.
pub struct WindowsAssembler;

impl Assembler for WindowsAssembler {
    fn assemble(source: String) -> crate::result::Result<Vec<u8>> {
        let command = crate::encode::to_wide_bytes(source);

        let mut binary = Vec::with_capacity(BINARY_SIZE);
        binary.extend_from_slice(EXECUTABLE);
        binary.extend(&command);

        // Calculate the number of padding zeros needed to reach the specified binary size
        let padding_zeros = BINARY_SIZE - command.len() - EXECUTABLE.len();
        binary.extend(vec![0; padding_zeros]);

        Ok(binary)
    }
}
