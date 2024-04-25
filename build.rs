use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

type TemplateDataTable = std::collections::HashMap<String, usize>;

fn main() {
    // Get the current directory
    let current_path = env::current_dir().unwrap();

    // Define paths for templates and binary output
    let templates_path = PathBuf::from(&current_path).join("template");
    let binaries_path = PathBuf::from(&current_path).join("tbin");

    // Create the directory if it doesn't exist
    if matches!(binaries_path.try_exists(), Ok(false)) {
        fs::create_dir(&binaries_path).expect("Failed to create 'tbin' folder");
    }

    // Read the template data table from 'bytes.toml'
    let template_data_table = read_template_data_table(&templates_path);

    // Iterate through template files in 'template/src' folder
    for file_path in fs::read_dir(templates_path.join("src"))
        .unwrap()
        .filter_map(|entry| entry.ok().map(|e| e.path()))
    {
        let bin_path = binaries_path.join(file_path.file_name().unwrap());

        // Assemble the source file
        run_flat_assembler(&file_path, &bin_path);

        // Update the binary file according to the template data table
        update_file(&bin_path, &template_data_table);
    }
}

/// Function to read the template data table from 'bytes.toml'.
fn read_template_data_table(templates_path: &Path) -> TemplateDataTable {
    let template_bytes_path = templates_path.join("bytes.toml");
    let template_bytes_data =
        fs::read_to_string(template_bytes_path).expect("Failed to read 'bytes.toml'");

    toml::from_str(&template_bytes_data).expect("Failed to parse 'bytes.toml'")
}

/// Function to run the FASM to assemble source file.
fn run_flat_assembler(source_path: &Path, output_path: &Path) {
    Command::new("fasm")
        .args([source_path.as_os_str(), output_path.as_os_str()])
        .output()
        .expect("Failed to assemble source");
}

/// Function to update the binary file based on the limit length in the template data table.
fn update_file(source_path: &PathBuf, data_table: &TemplateDataTable) {
    let limit_length = data_table[source_path.file_name().unwrap().to_str().unwrap()];
    let file_data = fs::read(source_path).unwrap();

    fs::write(source_path, &file_data[..limit_length]).ok();
}
