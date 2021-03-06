use crate::lib::{constants::LOG_FILE_PATH, errors::AppError, types::Result};
use log::LevelFilter;
use simplelog::*;
use std::{
    fs::{create_dir_all, File},
    path::Path,
    time::SystemTime,
};

fn get_log_file_path() -> Result<String> {
    Ok(format!(
        "{}{}.log",
        LOG_FILE_PATH,
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs()
    ))
}

pub fn initialize_file_logger() -> Result<()> {
    let log_path = get_log_file_path()?;
    if !Path::new(&LOG_FILE_PATH).exists() {
        info!("✔ No log dir found, creating...");
        create_dir_all(&LOG_FILE_PATH)?;
    };
    match WriteLogger::init(LevelFilter::Trace, Config::default(), File::create(log_path.clone())?) {
        Ok(_) => {
            info!("✔ Logger initialized successfully");
            info!("✔ Log writing to: {}", log_path);
            Ok(())
        },
        Err(e) => Err(AppError::Custom(e.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_log_path() {
        let result = get_log_file_path().unwrap();
        assert!(result.contains("logs/"));
    }
}
