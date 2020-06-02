pub fn incremental_backup() -> std::io::Result<()> {
    use std::io::{Read as _, Write as _};

    debug!("Starting settings incremental backup...");

    let mut apex_dir = crate::apex::apex_user_dir()?;
    let mut backup_dir = apex_dir.clone();

    apex_dir.push("local");
    backup_dir.push("ao_cfg_backups");

    debug!("Making sure backup dir is created at {:?}", backup_dir);
    std::fs::create_dir_all(backup_dir.clone())?;

    let mut backup_id = format!("backup-{}", chrono::Utc::now().format("%Y-%m-%d"));
    let mut counter = 1u16;

    for file in std::fs::read_dir(backup_dir.clone())?.filter_map(Result::ok) {
        if let Ok(file_name) = file.file_name().into_string() {
            if file_name.find(&backup_id).is_some() {
                counter += 1;
            }
        }
    }

    backup_id.push_str(&format!("-{:03}.zip", counter));
    debug!("About to create backup file {}", backup_id);

    backup_dir.push(backup_id);

    let zipfile = std::fs::File::create(backup_dir)?;
    let mut zip = zip::ZipWriter::new(zipfile);
    zip.set_comment(r#"
Created with apex-optimizer.
You can unzip this file and copy/paste this configuration to restore what you had at the date of the backup.
    "#);
    let mut buf = vec![];

    for apex_cfg_file in std::fs::read_dir(apex_dir.clone())?.filter_map(Result::ok) {
        if let Ok(file_name) = apex_cfg_file.file_name().into_string() {
            zip.start_file(file_name, zip::write::FileOptions::default())?;
            let mut file_path = apex_dir.clone();
            file_path.push(apex_cfg_file.file_name());
            debug!("Zipping {:?}...", file_path);
            std::fs::File::open(file_path)?.read_to_end(&mut buf)?;
            zip.write_all(&*buf)?;
            buf.clear();
        }
    }

    zip.finish()?;
    debug!("Backup completed without errors");

    Ok(())
}
