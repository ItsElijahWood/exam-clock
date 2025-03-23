use std::{fs::{self, File}, io::{self, Read, Write}, path::Path};

pub fn read_centre_file() -> String {
  let file_path = "centre_no.txt";

  if Path::new(file_path).exists() {
    read_file(file_path).unwrap()
  } else {
    create_file(&file_path).unwrap()
  }
}

fn read_file(file_path: &str) -> io::Result<String> {
  let mut read_file = fs::File::open(file_path)?;
  let mut contents = String::new();

  read_file.read_to_string(&mut contents)?;

  Ok(contents)
}

fn create_file(file_path: &str) -> io::Result<String> {
  let mut file_created = File::create(file_path)?;

  let listener = io::stdin();
  let mut input = String::new();

  println!("A file was not found. I've created a file for you called {} what centre number would you like to put in that file:", file_path);

  listener.read_line(&mut input)?;

  file_created.write(input.as_bytes()).expect("Error writing to file in read_centre_file.rs");
  Ok(input)
}