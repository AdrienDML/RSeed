use std::time::{SystemTime, SystemTimeError};

pub fn get_time() -> Result<String, SystemTimeError> {
    let total_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let mins_tot = total_time / 60;
    let hours_tot = mins_tot / 60;
    let secs = total_time - mins_tot * 60;
    let mins = mins_tot - hours_tot * 60;
    let hours = hours_tot - (hours_tot / 24) * 24;
    Ok(format!("{}:{}:{}", hours, mins, secs))
}
