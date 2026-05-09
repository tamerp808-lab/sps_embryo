use std::io::{self, Write};

pub fn run_cli() {
    println!("SPS Embryo CLI. اكتب 'exit' للخروج.");
    loop {
        print!("sps> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "exit" {
            break;
        }
        if input.is_empty() {
            continue;
        }
        let body = serde_json::json!({"goal": input}).to_string();
        let output = std::process::Command::new("curl")
            .args([
                "-s",
                "-X",
                "POST",
                "http://localhost:9101/execute",
                "-H",
                "Content-Type: application/json",
                "-d",
                &body,
            ])
            .output();
        match output {
            Ok(out) => {
                println!("{}", String::from_utf8_lossy(&out.stdout));
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}
