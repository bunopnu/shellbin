use crate::assembler::Assembler;

/// Represents the executable data.
const EXECUTABLE: &[u8; 257] = include_bytes!("../../../tbin/linux-amd64.s");

/// A struct representing a Linux-specific (amd64) assembler.
pub struct LinuxAssembler;

impl Assembler for LinuxAssembler {
    fn assemble(source: String) -> crate::result::Result<Vec<u8>> {
        let mut binary = Vec::with_capacity(EXECUTABLE.len() + source.len() + 1);

        binary.extend_from_slice(EXECUTABLE);
        binary.extend(source.bytes());
        binary.push(0);

        Ok(binary)
    }
}
