use anyhow::{Context, Ok, Result};
use clap::Parser;
use indicatif::ProgressBar;
use std::time::Duration;
use std::thread;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result, &ProgressBar::new(2)).unwrap();
    assert_eq!(result, b"lorem ipsum\n");
}
#[test]
fn test_progress_bar() {
    let total_tasks = 100;
    let mut progress_bar = ProgressBar::new(total_tasks as u64);
    progress_bar.set_message("Processing tasks...");

    for _ in 0..total_tasks {
        // Simulate a long-running task
        thread::sleep(Duration::from_millis(10));
        progress_bar.inc(1);
    }

    progress_bar.finish();
    assert_eq!(progress_bar.position(), total_tasks as u64);
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    let total_lines = content.lines().count();
    let pb = ProgressBar::new(total_lines as u64);
    pb.set_message("Processing lines...");

    let mut result = Vec::new();
    find_matches(&content, &args.pattern, &mut result, &pb)?;

    pb.finish();
    Ok(())
}

fn find_matches(
    content: &str,
    pattern: &str,
    mut writer: impl std::io::Write,
    pb: &ProgressBar,
) -> Result<()> {
    for (i, line) in content.lines().enumerate() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
        pb.set_position((i + 1) as u64); // Update the progress bar
    }
    Ok(())
}