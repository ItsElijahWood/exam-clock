use chrono::Local;

#[allow(dead_code)]
pub fn get_system_time() -> String {
  let time_now = Local::now();
  let format_time = time_now.format("%H:%M").to_string();

  format_time
}
