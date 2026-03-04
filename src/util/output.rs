use colored::Colorize;

pub fn success(msg: &str) {
    println!("{} {}", "✓".green(), msg);
}

pub fn warning(msg: &str) {
    println!("{} {}", "⚠".yellow(), msg);
}

pub fn error(msg: &str) {
    eprintln!("{} {}", "✗".red(), msg);
}

pub fn info(msg: &str) {
    println!("{} {}", "→".cyan(), msg);
}

pub fn neutral(msg: &str) {
    println!("{} {}", "–".dimmed(), msg);
}
