use std::env;
use walkdir::WalkDir;

use std::path::Path;

use std::fs;


fn main() -> std::io::Result<()> {
    let path = env::current_dir()?;
    let _pwd = path.display().to_string();
    
    for entry in WalkDir::new(&_pwd) {
        let path = entry?.path().display().to_string();

        let is_it_file = Path::new(&path).is_file();
        
        if is_it_file == true{
            let contents = fs::read_to_string(&path).expect("stream did not contain valid UTF-8");
            println!("{}", contents);
        }
    }
    Ok(())
}
