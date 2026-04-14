use colored::*;

pub fn print_header(name: &str, version: &str) {
    println!("{} v{}", name.cyan().bold(), version.cyan().bold());
    println!(
        "{}",
        crate::constants::APP_SEPARATOR_CHAR
            .repeat(crate::constants::APP_SEPARATOR_LEN)
            .cyan()
    );
}

pub fn success(msg: &str) {
    println!("{}", msg.green().bold());
}

pub fn error_msg(msg: &str) {
    eprintln!("{} {}", crate::constants::MSG_ERROR_PREFIX.red(), msg);
}

pub fn log_processing(path: &str, lang_name: &str) {
    println!(
        "{} {} ({})",
        crate::constants::MSG_PROCESSING_PREFIX.cyan(),
        path,
        lang_name.yellow()
    );
}

pub fn log_skip(path: &str) {
    println!(
        "{} {}",
        crate::constants::MSG_SKIPPING_PREFIX.yellow(),
        path.dimmed()
    );
}

pub fn log_backup(path: &str) {
    println!("{} {}", crate::constants::MSG_BACKUP_PREFIX.green(), path);
}

pub fn log_output(path: &str) {
    println!("{} {}", crate::constants::MSG_OUTPUT_PREFIX.green(), path);
}

pub fn log_dry_run() {
    println!("{}", crate::constants::MSG_DRY_RUN_SKIP.dimmed());
}

pub fn log_summary(processed: usize, skipped: usize) {
    println!(
        "\n{} {} {}, {} {} {}",
        crate::constants::MSG_PROCESSED_SUMMARY.green(),
        processed,
        crate::constants::MSG_FILES_SUFFIX,
        crate::constants::MSG_SKIPPED_SUMMARY.yellow(),
        skipped,
        crate::constants::MSG_FILES_SUFFIX
    );
}
