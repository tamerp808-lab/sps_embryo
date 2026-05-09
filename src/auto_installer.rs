use std::process::Command;

pub fn check_and_install_tools() -> Vec<String> {
    let required = [
        "espeak-ng",
        "tesseract",
        "ffmpeg",
        "whisper",
        "curl",
        "arecord",
    ];
    let mut missing = Vec::new();
    for tool in &required {
        if Command::new("which")
            .arg(tool)
            .output()
            .map(|o| !o.status.success())
            .unwrap_or(true)
        {
            missing.push(tool.to_string());
        }
    }
    if !missing.is_empty() {
        println!(
            "Missing tools: {:?}. Please install them manually.",
            missing
        );
    }
    missing
}
