use std::fs;
use std::io;

fn main() -> std::io::Result<()> {

    println!("Give me a file: ");

    let mut dir_path = String::new();

    io::stdin()
	.read_line(&mut dir_path)
	.expect("Failed to readline!");

    println!("dir choosen: {dir_path}");
    
    for entry in fs::read_dir(dir_path.trim())? {
	let dir = entry?;
	println!("{:?}", dir.path());
	
    }
    Ok(())
}
