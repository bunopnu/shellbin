use std::{fs::File, io::Write};

use assembler::{windows::WindowsAssembler, Assembler};

/// This module contains functionality related to generating executable binary.
mod assembler;

fn main() -> anyhow::Result<()> {
    let script = "echo Unpacking... && tar -xf main.zip && echo Unpacked!".to_string();

    let binary = WindowsAssembler::assemble(script)?;

    let mut file = File::create("./testing/output.exe")?;

    file.write_all(&binary)?;

    println!("Bytes have been written to the file!");

    Ok(())
}
