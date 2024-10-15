use crate::cronjob::CronJob;
use std::{
    process::Command,
    str,
    sync::{Arc, Mutex},
};

pub struct Crontab {
    jobs: Vec<CronJob>,
    lines: Vec<String>,
}

impl Crontab {
    // 获取单例实例的方法

    pub fn get_instance() -> Arc<Mutex<Crontab>> {
        static mut INSTANCE: Option<Arc<Mutex<Crontab>>> = None;
        unsafe {
            INSTANCE
                .get_or_insert_with(|| {
                    Arc::new(Mutex::new(Crontab {
                        jobs: vec![],
                        lines: vec![],
                    }))
                })
                .clone()
        }
    }
}

impl Crontab {
    pub fn load(&mut self) -> Result<Vec<CronJob>, String> {
        // Execute the `crontab -l` command
        let output = Command::new("crontab")
            .arg("-l")
            .output()
            .map_err(|err| format!("Failed to execute crontab command: {}", err))?;

        // Check if the command was successful
        if output.status.success() {
            // Convert the output to a string and split it into lines (jobs)
            let stdout = str::from_utf8(&output.stdout)
                .map_err(|err| format!("Failed to parse crontab output: {}", err))?;

            // Split lines, filter out comments/empty lines, and collect into a Vec<String>
            let lines: Vec<String> = stdout
                .lines()
                .filter(|line| !line.trim().is_empty())
                .map(|line| line.to_string())
                .collect();
            self.lines = lines.clone();
            let mut jobs: Vec<CronJob> = vec![];
            for line in lines {
                jobs.push(CronJob::new(line));
            }
            self.jobs = jobs.clone();
            Ok(jobs)
        } else {
            // Capture error output if the command fails
            let stderr = str::from_utf8(&output.stderr)
                .map_err(|err| format!("Failed to read error message: {}", err))?;
            Err(stderr.to_string())
        }
    }

    pub fn save(&mut self, jobs: Vec<CronJob>) -> Result<(), String> {
        // Convert the jobs into a Vec<String>
        let lines: Vec<String> = jobs.iter().map(|job| job.to_string()).collect();
        self.lines = lines.clone();
        // Join the lines into a single string
        let lines = lines.join("\n");
        // Execute the crontab command to update the crontab with the new jobs
        let mut child = std::process::Command::new("crontab")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .map_err(|err| format!("Failed to execute crontab command: {}", err))?;

        // Write the jobs (as string) to crontab's stdin
        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            stdin
                .write_all(lines.as_bytes())
                .map_err(|err| format!("Failed to write to crontab stdin: {}", err))?;
        }

        // Wait for the crontab command to finish
        let output = child
            .wait()
            .map_err(|err| format!("Failed to wait on crontab process: {}", err))?;

        if output.success() {
            Ok(())
        } else {
            Err("Failed to update crontab".to_string())
        }
    }
}
