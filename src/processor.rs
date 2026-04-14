use crate::cleaner;
use crate::constants;
use crate::language::Language;
use crate::logger;
use crate::Cli;
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn process(path: &Path, cli: &Cli) -> Result<()> {
    if path.is_file() {
        process_file(path, cli)?;
    } else if path.is_dir() {
        process_directory(path, cli)?;
    } else {
        anyhow::bail!("{} {}", constants::ERR_PATH_NOT_EXIST, path.display());
    }
    Ok(())
}

fn process_directory(dir: &Path, cli: &Cli) -> Result<()> {
    let extensions: Option<Vec<String>> = cli
        .extensions
        .as_ref()
        .map(|exts| exts.split(',').map(|s| s.trim().to_string()).collect());

    let walker = if cli.recursive {
        WalkDir::new(dir).into_iter()
    } else {
        WalkDir::new(dir).max_depth(1).into_iter()
    };

    let mut processed_count = 0;
    let mut skipped_count = 0;

    for entry in walker.filter_map(|e| e.ok()) {
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        if let Some(ref exts) = extensions {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if !exts.contains(&ext.to_string()) {
                    skipped_count += 1;
                    continue;
                }
            } else {
                skipped_count += 1;
                continue;
            }
        }

        let language = if let Some(ref lang_str) = cli.language {
            Language::from_str(lang_str)
        } else {
            Language::from_path(path)
        };

        if language.is_none() {
            logger::log_skip(&path.display().to_string());
            skipped_count += 1;
            continue;
        }

        match process_single_file(path, Some(dir), cli, language.unwrap()) {
            Ok(_) => {
                processed_count += 1;
            }
            Err(e) => {
                logger::error_msg(&format!("{}: {}", path.display(), e));
            }
        }
    }

    logger::log_summary(processed_count, skipped_count);

    Ok(())
}

fn process_file(file: &Path, cli: &Cli) -> Result<()> {
    let language = if let Some(ref lang_str) = cli.language {
        Language::from_str(lang_str).context(constants::ERR_UNRECOGNIZED_LANG_CLI)?
    } else {
        Language::from_path(file).context(constants::ERR_UNRECOGNIZED_LANG_EXT)?
    };

    process_single_file(file, None, cli, language)
}

fn process_single_file(
    file: &Path,
    base_dir: Option<&Path>,
    cli: &Cli,
    language: Language,
) -> Result<()> {
    logger::log_processing(&file.display().to_string(), language.name());

    let content = fs::read_to_string(file).context(format!(
        "{} {}",
        constants::ERR_READ_FILE,
        file.display()
    ))?;

    let cleaned_content = cleaner::clean_comments(&content, language);

    if cli.dry_run {
        logger::log_dry_run();
        return Ok(());
    }

    let output_path = determine_output_path(file, base_dir, cli)?;

    if let Some(parent) = output_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    if cli.backup && cli.in_place {
        let backup_path = file.with_extension(format!(
            "{}.bak",
            file.extension().and_then(|s| s.to_str()).unwrap_or("")
        ));
        fs::copy(file, &backup_path).context(format!(
            "{} {}",
            constants::ERR_CREATE_BACKUP,
            backup_path.display()
        ))?;

        logger::log_backup(&backup_path.display().to_string());
    }

    fs::write(&output_path, cleaned_content).context(format!(
        "{} {}",
        constants::ERR_WRITE_FILE,
        output_path.display()
    ))?;

    logger::log_output(&output_path.display().to_string());

    Ok(())
}

fn determine_output_path(file: &Path, base_dir: Option<&Path>, cli: &Cli) -> Result<PathBuf> {
    if cli.in_place {
        Ok(file.to_path_buf())
    } else if let Some(ref output) = cli.output {
        if cli.recursive {
            if let Some(base) = base_dir {
                let rel_path = file.strip_prefix(base).unwrap_or(file);
                Ok(output.join(rel_path))
            } else {
                let file_name = file.file_name().context(constants::ERR_GET_FILENAME)?;
                Ok(output.join(file_name))
            }
        } else {
            Ok(output.clone())
        }
    } else {
        let parent = file.parent().unwrap_or_else(|| Path::new("."));
        let stem = file
            .file_stem()
            .and_then(|s| s.to_str())
            .context(constants::ERR_GET_FILENAME)?;
        let extension = file.extension().and_then(|s| s.to_str()).unwrap_or("");

        let output_name = if extension.is_empty() {
            format!("{}_cleaned", stem)
        } else {
            format!("{}_cleaned.{}", stem, extension)
        };

        Ok(parent.join(output_name))
    }
}
