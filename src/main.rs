use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::Path;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <directory_path>", args[0]);
        return Ok(());
    }

    let dir_path = &args[1];

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let mut file = fs::File::open(&path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            let code_block_type = if path.extension().map_or(false, |ext| ext == "rs") {
                let filename = path.file_name().unwrap().to_string_lossy();
                format!("rust\n// {}\n", filename)
            } else {
                String::new()
            };

            println!("```{}", code_block_type);
            println!("{}", contents);
            println!("```");
        }
    }

    Ok(())
}
