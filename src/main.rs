use std::io::Read;
use std::fs::File;
use std::path::Path;
use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        panic!("Please provide a file");
    }
    let path = Path::new(&args[1]);
    let display = path.display();
    
    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // put all file content to a string
    let mut buf = String::new();
    let _source = match file.read_to_string(&mut buf) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(source) => source,
    };

}