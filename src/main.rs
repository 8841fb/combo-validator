use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::sync::{Arc, Mutex};

use regex::Regex;
use std::collections::HashSet;

fn validate_email(email: &str) -> bool {
    let email_regex = Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$").unwrap();
    email_regex.is_match(email)
}

fn process_file(file_name: &str, valid_emails: Arc<Mutex<HashSet<String>>>) {
    let file = File::open(file_name).expect("Failed to open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() == 2 {
                let email = parts[0];
                let password = parts[1];

                let password: Vec<&str> = password.splitn(2, "	").collect();
                let password = password[0];

                if password.len() >= 6 && validate_email(email) {
                    valid_emails
                        .lock()
                        .unwrap()
                        .insert(format!("{}:{}", email, password));
                }
            }
        }
    }
}

fn main() {
    let file_name = std::env::args().nth(1).expect("Missing file name");

    let valid_emails: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));
    process_file(&file_name, valid_emails.clone());

    let valid_emails = valid_emails.lock().unwrap();
    let mut output_file = File::create("valid.txt").expect("Failed to create output file");

    let emails_string = valid_emails.iter().fold(String::new(), |mut acc, email| {
        acc.push_str(email);
        acc.push('\n');
        acc
    });
    output_file
        .write_all(emails_string.as_bytes())
        .expect("Failed to write to output file");

    println!("done | {}", file_name);
}
