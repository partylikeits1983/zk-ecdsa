use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

fn covert() -> io::Result<()> {
    let hash_path = Path::new("data/hash.txt");
    let publickeyx_path = Path::new("data/publicKeyX.txt");
    let publickeyy_path = Path::new("data/publicKeyY.txt");
    let signature_path = Path::new("data/signature.txt");

    let output_path = Path::new("circuits/Prover.toml");

    // Open the output file for writing, create it if it doesn't exist, or truncate it if it does
    let mut output_file = File::create(&output_path)?;

    // Initialize variables for leaf and root values

    let hash_value = read_first_line(&hash_path)?;
    let publickeyx_value = read_first_line(&publickeyx_path)?;
    let publickeyy_value = read_first_line(&publickeyy_path)?;
    let signature_value = read_first_line(&signature_path)?;

    // Write leaf and root
    writeln!(output_file, "hashed_message = \"{}\"", hash_value)?;
    writeln!(output_file, "publicKeyX = \"{}\"", publickeyx_value)?;  
    writeln!(output_file, "publicKeyY = \"{}\"", publickeyy_value)?;  
    writeln!(output_file, "signature = \"{}\"", signature_value)?;
    
    Ok(())
}

// Helper function to read the first line from a file
fn read_first_line(path: &Path) -> io::Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = io::BufReader::new(file);
    let mut line = String::new();
    // Only read the first line
    buf_reader.read_line(&mut line)?;
    // Trim the newline character(s) at the end of the line
    Ok(line.trim_end().to_string())
}
