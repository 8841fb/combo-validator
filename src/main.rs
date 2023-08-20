use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::sync::{Arc, Mutex};
use std::thread;

use regex::Regex;

fn validate_email(email: &str) -> bool {
    let email_regex = Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$").unwrap();
    email_regex.is_match(email)
}

fn process_file(file_name: &str, valid_emails: Arc<Mutex<Vec<String>>>) {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() == 2 {
                let email = parts[0];
                let password = parts[1];

                // if theres "	" in the password, remove it and everything after it
                if let Some(index) = password.find("	") {
                    let password = &password[..index];
                }

                if password.len() < 6 {
                    continue;
                }

                if validate_email(email) {
                    let valid_email = format!("{}:{}", email, password);
                    valid_emails.lock().unwrap().push(valid_email);
                }
            }
        }
    }
}

fn main() {
    let file_name = std::env::args().nth(1).expect("Missing file name");

    let valid_emails: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let mut threads = vec![];

    for _ in 0..num_cpus::get() {
        let file_name = file_name.clone();
        let valid_emails = valid_emails.clone();

        let thread = thread::spawn(move || {
            process_file(&file_name, valid_emails);
        });

        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }

    let valid_emails = valid_emails.lock().unwrap();
    let mut output_file = File::create("valid_emails.txt").unwrap();

    for email in valid_emails.iter() {
        writeln!(output_file, "{}", email).unwrap();
    }

    println!("done | {}", file_name);
}