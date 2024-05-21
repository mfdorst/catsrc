use std::{env, fs, io};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <directory_path>", args[0]);
        return Ok(());
    }

    let dir_path = &args[1];

    let ignored_files = vec![".gitignore"];
    let ignored_extensions = vec!["lock"];

    for entry in fs::read_dir(dir_path)? {
        let path = entry?.path();
        let extension = path.extension().map(|ext| ext.to_str().unwrap());
        let filename = path.file_name().unwrap().to_string_lossy().into_owned();

        let is_ignored = ignored_extensions.contains(&extension.unwrap_or_default())
            || ignored_files.contains(&filename.as_str());

        if path.is_file() && !is_ignored {
            let contents = fs::read_to_string(&path)?;

            let (lang, comment_start, comment_end) = match extension {
                Some("rs") => ("rust", "//", ""),
                Some("sh") => ("bash", "#", ""),
                Some("py") => ("python", "#", ""),
                Some("js") => ("javascript", "//", ""),
                Some("ts") => ("typescript", "//", ""),
                Some("jsx") => ("jsx", "//", ""),
                Some("html") => ("html", "<!--", " -->"),
                Some("css") => ("css", "/*", " */"),
                Some("nix") => ("nix", "#", ""),
                _ => ("", "# ", ""),
            };

            println!("```{lang}");
            println!("{comment_start} {filename}{comment_end}");
            println!("{contents}```");
        }
    }

    Ok(())
}
