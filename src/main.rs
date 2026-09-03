use std::fs;
use std::io;

fn main() -> std::io::Result<()> {

    println!("Give me a file: ");

    let mut dir_path = String::new();

    io::stdin()
	.read_line(&mut dir_path)
	.expect("Failed to readline!");

    let trimmed_path = dir_path.trim();
    println!("dir choosen: {trimmed_path}");
    
    let mut pdf_list: Vec<(String, u64)> = Vec::new();

    for entry in fs::read_dir(trimmed_path)? {
	let dir = entry?;
	let path = dir.path();

	// check and extract the extension file
	if let Some(ext) = path.extension() {
	    if ext.eq_ignore_ascii_case("pdf") {

		//lets get the metadat if its a pdf
		let file_size = dir.metadata()?.len();
		
		if let Some(path_str) = path.to_str() {

		    // we also needs to add the metadata to this, otherwise wont get in the list
		    pdf_list.push((path_str.to_string(), file_size));

		}

	    }

	}


	}
	

    println!("\nFound PDFs (Path, Size in Bytes): ");
    for (path, size) in &pdf_list {
	println!("{} - {} bytes", path, size);
	
    }
    Ok(())
}
