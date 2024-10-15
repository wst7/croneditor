
use tauri::{command, AppHandle};

extern crate rust_crontab;
use rust_crontab::{cronjob::CronJob, crontab::Crontab};

#[command]
pub fn load_crons(app: AppHandle) -> Result<Vec<CronJob>, String> {
  let ins = Crontab::get_instance();
  let mut crontab_instance = ins.lock().map_err(|err| format!("Failed to lock crontab instance: {}", err))?;
  let jobs = crontab_instance.load()?;
  Ok(jobs)
}

#[command]
pub fn save_crons(app: AppHandle, jobs: Vec<CronJob>) -> Result<(), String> {
  let ins = Crontab::get_instance();
  let mut crontab_instance = ins.lock().map_err(|err| format!("Failed to lock crontab instance: {}", err))?;
  crontab_instance.save(jobs)?;
  Ok(())
}