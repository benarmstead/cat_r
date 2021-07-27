use std::fs;
use std::env;
use std::path::Path;
use walkdir::WalkDir;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let pwd: String;

    if args.len() < 2 {
        let path = env::current_dir()?;
        pwd = path.display().to_string();
    }
    else{
        pwd = args[1].to_string();
    }

    for entry in WalkDir::new(&pwd) {
        let path = entry?.path().display().to_string();

        let is_it_file = Path::new(&path).is_file();
        
        if is_it_file == true{
            let contents = fs::read_to_string(&path).expect("stream did not contain valid UTF-8");
            println!("{}", contents);
        }
    }
    Ok(())
}
