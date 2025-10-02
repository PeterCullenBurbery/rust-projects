use std::fs::{self, File};
use std::io::{self};
use std::process::{Command, Stdio};
use std::path::{Path, PathBuf};

/// Build a Champernowne number string until we exceed a target length
fn build_champernowne(limit: usize) -> String {
    let mut s = String::with_capacity(limit);
    let mut n = 1;
    while s.len() < limit {
        s.push_str(&n.to_string());
        n += 1;
    }
    s
}

fn main() -> io::Result<()> {
    // Generate Champernowne digits until at least 400 chars
    let champernowne = build_champernowne(400);

    // Construct a very long filename
    let long_name = format!("champernowne_test_{}.mp4", champernowne);

    // Base directory
    let base_dir = Path::new(r"C:\data\videos");

    // Ensure directory exists
    fs::create_dir_all(base_dir)?;

    // Join path cleanly (no accidental double separators)
    let normal_path: PathBuf = base_dir.join(&long_name);

    // Extended-length version if too long
    let full_path = if normal_path.to_string_lossy().len() > 250 {
        // Prepend \\?\ and keep backslashes
        format!(r"\\?\{}", normal_path.display().to_string().replace("/", r"\"))
    } else {
        normal_path.display().to_string()
    };

    println!("Filename length: {}", long_name.len());
    println!("Full path length: {}", full_path.len());
    println!("Recording to: {}", full_path);

    // Create file manually
    let file = File::create(&full_path)?;

    // Spawn ffmpeg to stdout
    let mut child = Command::new("ffmpeg")
        .args(&[
            "-y",
            "-f", "lavfi",
            "-i", "testsrc=duration=20:size=128x128:rate=15",
            "-c:v", "libx264",
            "-pix_fmt", "yuv420p",
            "-f", "mp4",
            "pipe:1",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to spawn ffmpeg");

    // Pipe stdout into our long-path file
    if let Some(mut stdout) = child.stdout.take() {
        let mut writer = io::BufWriter::new(file);
        io::copy(&mut stdout, &mut writer)?;
    }

    let status = child.wait().expect("Failed to wait for ffmpeg");
    if !status.success() {
        eprintln!("ffmpeg failed with status: {:?}", status.code());
    } else {
        println!("Recording succeeded!");
    }

    Ok(())
}