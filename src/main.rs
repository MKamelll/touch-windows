// Use
use std::env;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};

// Main
fn main() {
  // Get files names
  let mut args: Vec<String> = env::args().collect();
  args.remove(0);

  // Get current directory
  let cuurent_directory_buf = env::current_dir()
                                  .expect("Could not get current working directory");
  let cuurent_directory_path = Path::new(&cuurent_directory_buf);
  
  //println!("{:?}", args);
  for file_name in args {
    if is_file(&file_name, &cuurent_directory_path) == false {
      create_file(&file_name, &cuurent_directory_path);

    } else {
      println!("File already exists!");
    }
  }
    println!("Hello, world!");
}

// Check if file exists
fn is_file (file_name: &String, working_dir: &Path) -> bool {
  let file_path = working_dir.join(file_name);
  if file_path.is_file() == true {
    return true;
  }
  false
}

// Create files
fn create_file (file_name: &String, cuurent_directory_path: &Path) {
  let new_file_path = cuurent_directory_path.join(file_name);
  let error = &str From format!("Could not create {:?}", new_file_path);
  File::create(new_file_path)
        .expect();
}