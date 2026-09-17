use std::{
    fs,
    path::{Path, PathBuf},
};

use utils::{get_location, Error};

/// Official Quantframe Tauri identifier / LocalAppData folder name.
pub const QUANTFRAME_APP_PATH: &str = "dev.kenya.quantframe";
/// Kuantframe Tauri identifier / LocalAppData folder name.
pub const KUANTFRAME_APP_PATH: &str = "dev.kuantframe";

const PENDING_IMPORT_DIR: &str = "dev.kuantframe.pending-import";
const PENDING_IMPORT_FLAG: &str = "dev.kuantframe.pending-import.flag";
const BACKUP_DIR_PREFIX: &str = "dev.kuantframe.backup-";
const QUANTFRAME_DB_STEM: &str = "quantframeV2";
const KUANTFRAME_DB_STEM: &str = "kuantframeV2";

const SKIP_DIR_NAMES: &[&str] = &["EBWebView"];

fn save_error(message: impl Into<String>) -> Error {
    Error::new("SaveTransfer", message, get_location!())
}

fn io_error(path: &Path, action: &str, err: std::io::Error) -> Error {
    let path_buf = path.to_path_buf();
    Error::from_io("SaveTransfer", &path_buf, action, err, get_location!())
}

pub fn quantframe_path(local_data: &Path) -> PathBuf {
    local_data.join(QUANTFRAME_APP_PATH)
}

pub fn kuantframe_path(local_data: &Path) -> PathBuf {
    local_data.join(KUANTFRAME_APP_PATH)
}

pub fn pending_import_path(local_data: &Path) -> PathBuf {
    local_data.join(PENDING_IMPORT_DIR)
}

fn pending_flag_path(local_data: &Path) -> PathBuf {
    local_data.join(PENDING_IMPORT_FLAG)
}

/// True when the Quantframe app-data folder contains settings, auth, cache, or a database.
pub fn quantframe_dir_has_save(src: &Path) -> bool {
    if !src.is_dir() {
        return false;
    }
    for marker in ["settings.json", "auth.json", "cache_version.json"] {
        if src.join(marker).is_file() {
            return true;
        }
    }
    let cache = src.join("cache");
    if cache.is_dir() {
        if let Ok(mut entries) = fs::read_dir(&cache) {
            if entries.next().is_some() {
                return true;
            }
        }
    }
    match fs::read_dir(src) {
        Ok(entries) => entries.flatten().any(|entry| {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            name.starts_with(QUANTFRAME_DB_STEM) && entry.path().is_file()
        }),
        Err(_) => false,
    }
}

fn should_skip(name: &str) -> bool {
    SKIP_DIR_NAMES
        .iter()
        .any(|skip| name.eq_ignore_ascii_case(skip))
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), Error> {
    fs::create_dir_all(dst).map_err(|e| io_error(dst, "creating destination folder", e))?;
    let entries = fs::read_dir(src).map_err(|e| {
        io_error(
            src,
            "reading Quantframe data (close Quantframe if it is running)",
            e,
        )
    })?;
    for entry in entries {
        let entry = entry.map_err(|e| io_error(src, "reading Quantframe data", e))?;
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        if should_skip(&name_str) {
            continue;
        }
        let from = entry.path();
        let to = dst.join(&name);
        let file_type = entry
            .file_type()
            .map_err(|e| io_error(&from, "reading Quantframe entry type", e))?;
        if file_type.is_dir() {
            copy_dir_recursive(&from, &to)?;
        } else {
            if let Some(parent) = to.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| io_error(parent, "creating destination folder", e))?;
            }
            fs::copy(&from, &to).map_err(|e| {
                save_error(format!(
                    "Could not copy {}. Close Quantframe if it is running, then try again ({}).",
                    from.display(),
                    e
                ))
            })?;
        }
    }
    Ok(())
}

/// Rename `quantframeV2*` database files to `kuantframeV2*` in the app-data root.
pub fn rename_quantframe_db_files(dir: &Path) -> Result<(), Error> {
    let entries = fs::read_dir(dir).map_err(|e| io_error(dir, "listing database files", e))?;
    let mut renames: Vec<(PathBuf, PathBuf)> = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| io_error(dir, "listing database files", e))?;
        if !entry.path().is_file() {
            continue;
        }
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if let Some(rest) = name.strip_prefix(QUANTFRAME_DB_STEM) {
            let new_name = format!("{}{}", KUANTFRAME_DB_STEM, rest);
            renames.push((entry.path(), dir.join(new_name)));
        }
    }
    for (from, to) in renames {
        if to.exists() {
            fs::remove_file(&to)
                .map_err(|e| io_error(&to, "removing existing database file", e))?;
        }
        fs::rename(&from, &to).map_err(|e| io_error(&from, "renaming database file", e))?;
    }
    Ok(())
}

fn unique_backup_path(local_data: &Path) -> PathBuf {
    let ts = chrono::Local::now().format("%Y%m%d-%H%M%S");
    let mut backup = local_data.join(format!("{}{}", BACKUP_DIR_PREFIX, ts));
    let mut n = 1u32;
    while backup.exists() {
        backup = local_data.join(format!("{}{}-{}", BACKUP_DIR_PREFIX, ts, n));
        n += 1;
    }
    backup
}

/// Copy Quantframe app data into a pending folder and set a flag so the next launch can replace Kuantframe data.
///
/// The live Kuantframe database is left untouched until restart, because SQLite files are locked while the app runs.
pub fn stage_quantframe_import(local_data: &Path) -> Result<PathBuf, Error> {
    let src = quantframe_path(local_data);
    if !src.exists() {
        return Err(save_error(format!(
            "No Quantframe save found at {}. Install or run official Quantframe first, then try again.",
            src.display()
        )));
    }
    if !quantframe_dir_has_save(&src) {
        return Err(save_error(format!(
            "Quantframe folder exists at {} but no save data was found (settings, cache, login, or database).",
            src.display()
        )));
    }

    let staging = pending_import_path(local_data);
    if staging.exists() {
        fs::remove_dir_all(&staging)
            .map_err(|e| io_error(&staging, "clearing a previous pending import", e))?;
    }
    copy_dir_recursive(&src, &staging)?;
    rename_quantframe_db_files(&staging)?;
    fs::write(pending_flag_path(local_data), "quantframe-import")
        .map_err(|e| io_error(&pending_flag_path(local_data), "writing import flag", e))?;
    Ok(staging)
}

fn try_restore(backup: &Path, dest: &Path) {
    if dest.exists() {
        let _ = fs::remove_dir_all(dest);
    }
    let _ = fs::rename(backup, dest);
}

/// Apply a staged Quantframe import before the database and logger open.
///
/// Returns `Ok(Some(backup_path))` when Kuantframe data was replaced, `Ok(None)` when nothing was pending.
pub fn apply_pending_quantframe_import(local_data: &Path) -> Result<Option<PathBuf>, Error> {
    let flag = pending_flag_path(local_data);
    if !flag.exists() {
        return Ok(None);
    }
    let staging = pending_import_path(local_data);
    if !staging.exists() {
        let _ = fs::remove_file(&flag);
        return Err(save_error(
            "A Quantframe import was pending, but the staged save is missing. No data was changed.",
        ));
    }

    let dest = kuantframe_path(local_data);
    let mut backup_path = None;
    if dest.exists() {
        let backup = unique_backup_path(local_data);
        fs::rename(&dest, &backup).map_err(|e| {
            save_error(format!(
                "Could not back up current Kuantframe data at {} ({})",
                dest.display(),
                e
            ))
        })?;
        backup_path = Some(backup);
    }

    if let Err(e) = fs::rename(&staging, &dest) {
        if let Some(ref backup) = backup_path {
            try_restore(backup, &dest);
        }
        return Err(save_error(format!(
            "Could not replace Kuantframe data with the Quantframe save ({})",
            e
        )));
    }

    let _ = fs::remove_file(&flag);
    Ok(backup_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEST_SEQ: AtomicU64 = AtomicU64::new(0);

    fn test_root(name: &str) -> PathBuf {
        let n = TEST_SEQ.fetch_add(1, Ordering::SeqCst);
        let root = std::env::temp_dir().join(format!(
            "kf-save-transfer-{}-{}-{}",
            std::process::id(),
            n,
            name
        ));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();
        root
    }

    fn write_file(path: &Path, contents: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        let mut f = File::create(path).unwrap();
        f.write_all(contents.as_bytes()).unwrap();
    }

    #[test]
    fn missing_or_empty_dir_is_not_a_save() {
        let root = test_root("empty");
        let src = quantframe_path(&root);
        assert!(!quantframe_dir_has_save(&src));
        fs::create_dir_all(&src).unwrap();
        assert!(!quantframe_dir_has_save(&src));
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn settings_or_sqlite_count_as_save() {
        let root = test_root("markers");
        let src = quantframe_path(&root);
        fs::create_dir_all(&src).unwrap();
        write_file(&src.join("settings.json"), "{}");
        assert!(quantframe_dir_has_save(&src));

        fs::remove_file(src.join("settings.json")).unwrap();
        write_file(&src.join("quantframeV2.sqlite"), "db");
        assert!(quantframe_dir_has_save(&src));
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn stage_errors_when_quantframe_missing() {
        let root = test_root("missing");
        let err = stage_quantframe_import(&root).unwrap_err();
        assert!(err.message.contains("No Quantframe save found"));
        assert!(err.message.contains(QUANTFRAME_APP_PATH));
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn stage_errors_when_quantframe_has_no_save_files() {
        let root = test_root("nosave");
        fs::create_dir_all(quantframe_path(&root)).unwrap();
        let err = stage_quantframe_import(&root).unwrap_err();
        assert!(err.message.contains("no save data was found"));
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn stage_copies_save_renames_sqlite_and_skips_webview() {
        let root = test_root("stage");
        let src = quantframe_path(&root);
        write_file(&src.join("settings.json"), "{\"lang\":\"en\"}");
        write_file(&src.join("auth.json"), "{\"token\":\"secret\"}");
        write_file(&src.join("cache").join("items.json"), "[]");
        write_file(&src.join("quantframeV2.sqlite"), "db");
        write_file(&src.join("quantframeV2.sqlite-wal"), "wal");
        write_file(&src.join("quantframeV2.sqlite-shm"), "shm");
        write_file(&src.join("EBWebView").join("Cookies"), "should-not-copy");

        let staging = stage_quantframe_import(&root).unwrap();
        assert!(staging.join("settings.json").is_file());
        assert!(staging.join("auth.json").is_file());
        assert!(staging.join("cache").join("items.json").is_file());
        assert!(staging.join("kuantframeV2.sqlite").is_file());
        assert!(staging.join("kuantframeV2.sqlite-wal").is_file());
        assert!(staging.join("kuantframeV2.sqlite-shm").is_file());
        assert!(!staging.join("quantframeV2.sqlite").exists());
        assert!(!staging.join("EBWebView").exists());
        assert!(pending_flag_path(&root).is_file());
        // Source is left intact.
        assert!(src.join("quantframeV2.sqlite").is_file());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn apply_is_noop_without_flag() {
        let root = test_root("noop");
        assert!(apply_pending_quantframe_import(&root).unwrap().is_none());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn apply_replaces_dest_and_keeps_backup() {
        let root = test_root("apply");
        let src = quantframe_path(&root);
        write_file(&src.join("settings.json"), "{\"from\":\"quantframe\"}");
        write_file(&src.join("quantframeV2.sqlite"), "qf-db");

        let dest = kuantframe_path(&root);
        write_file(&dest.join("settings.json"), "{\"from\":\"kuantframe\"}");
        write_file(&dest.join("kuantframeV2.sqlite"), "kf-db");

        stage_quantframe_import(&root).unwrap();
        let backup = apply_pending_quantframe_import(&root)
            .unwrap()
            .expect("backup path");

        assert_eq!(
            fs::read_to_string(dest.join("settings.json")).unwrap(),
            "{\"from\":\"quantframe\"}"
        );
        assert_eq!(
            fs::read_to_string(dest.join("kuantframeV2.sqlite")).unwrap(),
            "qf-db"
        );
        assert!(!dest.join("quantframeV2.sqlite").exists());
        assert!(!pending_flag_path(&root).exists());
        assert!(!pending_import_path(&root).exists());
        assert_eq!(
            fs::read_to_string(backup.join("settings.json")).unwrap(),
            "{\"from\":\"kuantframe\"}"
        );
        assert_eq!(
            fs::read_to_string(backup.join("kuantframeV2.sqlite")).unwrap(),
            "kf-db"
        );
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn apply_clears_flag_when_staging_is_missing() {
        let root = test_root("missing-staging");
        fs::write(pending_flag_path(&root), "quantframe-import").unwrap();
        let dest = kuantframe_path(&root);
        write_file(&dest.join("settings.json"), "{\"keep\":true}");
        let err = apply_pending_quantframe_import(&root).unwrap_err();
        assert!(err.message.contains("staged save is missing"));
        assert!(!pending_flag_path(&root).exists());
        assert_eq!(
            fs::read_to_string(dest.join("settings.json")).unwrap(),
            "{\"keep\":true}"
        );
        let _ = fs::remove_dir_all(&root);
    }
}
