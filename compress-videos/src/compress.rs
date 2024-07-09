use std::fs::File;
use std::io::{BufReader, copy};
use std::error::Error;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::time::Instant;

pub fn compress_file(source: &str, target: &str) -> Result<(), Box<dyn Error>> {
    let input = File::open(source)?;
    let mut input = BufReader::new(input);
    let output = File::create(target)?;
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
    copy(&mut input, &mut encoder)?;
    let output = encoder.finish()?;
    println!("Target file size: {} bytes", input.get_ref().metadata()?.len());
    println!("Output file size: {} bytes", output.metadata()?.len());
    println!("Elapsed time: {:?}", start.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::{Write};
    use std::fs;

    #[test]
    fn test_compress_file_success() {
        // Create a temporary input file with some content
        let mut input_file = NamedTempFile::new().unwrap();
        writeln!(input_file, "This is a test file.").unwrap();

        // Create a temporary output file path
        let output_file = NamedTempFile::new().unwrap();
        let output_path = output_file.path().to_str().unwrap().to_string();

        // Call the compress_file function
        let result = compress_file(input_file.path().to_str().unwrap(), &output_path);
        assert!(result.is_ok());

        // Check if the output file is created
        let metadata = fs::metadata(&output_path).unwrap();
        assert!(metadata.is_file());

        // Check if the output file is not empty
        assert!(metadata.len() > 0);
    }

    #[test]
    fn test_compress_file_missing_source() {
        // Create a temporary output file path
        let output_file = NamedTempFile::new().unwrap();
        let output_path = output_file.path().to_str().unwrap().to_string();

        // Call the compress_file function with a non-existing source file
        let result = compress_file("non_existing_file.txt", &output_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_compress_file_unwritable_target() {
        // Create a temporary input file with some content
        let mut input_file = NamedTempFile::new().unwrap();
        writeln!(input_file, "This is a test file.").unwrap();

        // Call the compress_file function with an unwritable target file path
        let result = compress_file(input_file.path().to_str().unwrap(), "/non_writable_path/output.gz");
        assert!(result.is_err());
    }
}
