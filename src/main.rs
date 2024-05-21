use std::env;
use std::fs;
use std::io::{self, Read};

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

            let (language, comment_start, comment_end) =
                match path.extension().map(|ext| ext.to_str().unwrap_or("")) {
                    Some("rs") => ("rust", "//", ""),
                    Some("sh" | "bash") => ("bash", "#", ""),
                    Some("py") => ("python", "#", ""),
                    Some("js") => ("javascript", "//", ""),
                    Some("ts") => ("typescript", "//", ""),
                    Some("jsx") => ("jsx", "//", ""),
                    Some("html") => ("html", "<!--", " -->"),
                    Some("css") => ("css", "/*", " */"),
                    Some("nix") => ("nix", "#", ""),
                    _ => ("", "#", ""),
                };

            let filename = path.file_name().unwrap().to_string_lossy();

            println!("```{language}");
            println!("{comment_start} {filename}{comment_end}");
            println!("{contents}");
            println!("```");
        }
    }

    Ok(())
}
