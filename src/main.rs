// use std::fs::File;
use std::fs::OpenOptions;
// use std::io::Read;
use std::io::Write;
use chrono::prelude::*;

struct Entry{
    date: String,
    contents: String,
    completion: bool,
}

impl Entry{
    // Constructor Function.   
    fn new_entry (contents: String) -> Entry{
        let raw_date = Local::now();
        let date = raw_date.format("%Y-%m-%d %H:%M:%S").to_string();
        let completion = false;
        Entry{date, contents, completion}
    }

    fn create_entry_string (self) -> String{
        if self.completion == true {
            let completion_string = "✅ :".to_string();
            return format!("{} {}  -> Date added: {}", completion_string, &self.contents, &self.date);
        }
        else{
            let completion_string = "❎ :".to_string();
            return format!("{} {}  -> Date added: {}\n", completion_string, &self.contents, &self.date);
        }
    }
}



fn main() {
    let user_input = String::from("Yet another thing I wanna do!");
    let my_entry = Entry::new_entry(user_input);
    let entry_string = my_entry.create_entry_string();

    // Open a file with append option
    let mut memory = OpenOptions::new()
        .append(true)
        .open("memory.txt")
        .expect("cannot open file");

    // Write to a file
    memory
        .write(entry_string.as_bytes())
        .expect("write failed!");
}