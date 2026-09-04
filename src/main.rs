use std::fs;
use std::io;
use std::path::Path;
use clap::Parser;

#[derive(Parser)]
struct Args {
    dir: String,
}

// clap related function - takes command line dir path and checks if the dir exists
// if it exists we use it, if it does not we keep asking user for a dir till we receive a real dir
fn get_valid_dir() -> String {
    let args = Args::parse();
    let mut dir_path = args.dir.trim().to_string();

    loop {
        if Path::new(&dir_path).is_dir() {
            return dir_path;
        }

        println!("dir does not exist, give me a real dir: ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to readline!");
        dir_path = input.trim().to_string();
    }
}

// pure function - takes cleaned and proper dir path, reads the dir and returns only PDF paths + sizes
fn pdfs_in_dir(dir: &str) -> Vec<(String, u64)> {
    let mut pdf_list: Vec<(String, u64)> = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            if let Some(ext) = path.extension()
                && ext.eq_ignore_ascii_case("pdf")
                && let Ok(meta) = entry.metadata()
            {
                let file_size = meta.len();
                if let Some(path_str) = path.to_str() {
                    pdf_list.push((path_str.to_string(), file_size));
                }
            }
        }
    }

    pdf_list
}

fn main() -> std::io::Result<()> {
    let trimmed_path = get_valid_dir();
    println!("dir choosen: {trimmed_path}");

    let pdf_list = pdfs_in_dir(&trimmed_path);

    println!("\nFound PDFs (Path, Size in Bytes): ");
    for (path, size) in &pdf_list {
        println!("{} - {} bytes", path, size);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_pdfs_in_dir() {
        // use the testdata/ fixture relative to the crate root
        let mut fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        fixture.push("testdata");

        let pdfs = pdfs_in_dir(fixture.to_str().unwrap());

        // we expect exactly the three PDF files (case-insensitive extension)
        assert_eq!(pdfs.len(), 3);

        // all returned paths must end with .pdf (case-insensitive)
        for (path, _size) in &pdfs {
            let lower = path.to_lowercase();
            assert!(lower.ends_with(".pdf"));
        }
    }
}
