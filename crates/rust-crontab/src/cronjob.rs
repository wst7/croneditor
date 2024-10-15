use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CronJob {
    pub line: Option<String>,
    pub cron: String,
    pub command: String,
    pub comment: Option<String>,
    pub _disabled: bool,
}
impl CronJob {
    pub fn new(line: String) -> Self {
        let mut cron_job = CronJob {
            line: Some(line),
            cron: String::new(),
            command: String::new(),
            comment: None,
            _disabled: false,
        };

        // Parse the cron job string
        cron_job.parse();

        cron_job
    }
}

impl CronJob {
    pub fn parse(&mut self) -> &Self {
        let mut job = self.line.clone().unwrap();
        if job.trim_start().starts_with('#') {
            self._disabled = true;
            job = job.trim_start_matches('#').trim().to_string();
        }
        // Split the job by any `#` sign for potential comments
        let mut parts = job.splitn(2, '#');
        let cron_and_command = parts.next().unwrap_or("").trim();
        let comment = parts.next().unwrap_or("").trim().to_string();

        // Split the cron expression from the command
        let cron_expression_length = 5; // Expected number of fields in the cron expression

        let cron_and_command_parts: Vec<&str> = cron_and_command.split_whitespace().collect();

        // Ensure there are enough parts for the cron expression and command
        if cron_and_command_parts.len() >= cron_expression_length {
            // Separate the cron part and the command part
            self.cron = cron_and_command_parts[..cron_expression_length].join(" ");
            self.command = cron_and_command_parts[cron_expression_length..]
                .join(" ")
                .trim()
                .to_string();
        } else {
            // Handle case where there are not enough parts
            self.cron = "".to_string();
            self.command = "".to_string();
        }
        self.comment = Some(comment);
        self
    }

    pub fn render(&self) -> String {
        let mut job = format!("{} {}", self.cron, self.command);
        let comment = self.comment.clone().unwrap();
        // If there is a comment, append it with a leading `#`
        if !comment.is_empty() {
            job = format!("{} #{}", job, comment);
        }
        if self._disabled {
            job = format!("#{}", job);
        }

        job
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cron_job_new() {
        // Define a test cron job string with a comment
        let job_string = "*/20 * * * * . ~/.myenv; /usr/bin/python3 /Users/wstreet7/project/check_tasks/check_tasks.py; # This is a comment";

        // Create a new CronJob instance
        let cron_job = CronJob::new(job_string.to_string());

        // Define expected values
        let expected_cron = "*/20 * * * *".to_string();
        let expected_command =
            ". ~/.myenv; /usr/bin/python3 /Users/wstreet7/project/check_tasks/check_tasks.py;"
                .to_string(); // No extra space before ;
        let expected_comment = Some("This is a comment".to_string());

        // Assert that the parsed values match the expected values
        assert_eq!(cron_job.cron, expected_cron);
        assert_eq!(cron_job.command, expected_command);
        assert_eq!(cron_job.comment, expected_comment);
        assert_eq!(cron_job._disabled, false); // Not _disabled
    }

    #[test]
    fn test_cron_job__disabled() {
        // Test a _disabled cron job
        let job_string = "# */20 * * * * . ~/.myenv; /usr/bin/python3 /Users/wstreet7/project/check_tasks/check_tasks.py; # This is a comment";

        let cron_job = CronJob::new(job_string.to_string());

        let expected_cron = "*/20 * * * *".to_string();
        let expected_command =
            ". ~/.myenv; /usr/bin/python3 /Users/wstreet7/project/check_tasks/check_tasks.py;"
                .to_string();
        let expected_comment = Some("This is a comment".to_string());

        assert_eq!(cron_job.cron, expected_cron);
        assert_eq!(cron_job.command, expected_command);
        assert_eq!(cron_job.comment, expected_comment);
        assert_eq!(cron_job._disabled, true); // This job should be marked as _disabled
    }

    #[test]
    fn test_cron_job_without_comment() {
        // Test a cron job without a comment
        let job_string = "*/20 * * * * . ~/.myenv; /usr/bin/python3 /Users/wstreet7/project/check_tasks/check_tasks.py;";

        let cron_job = CronJob::new(job_string.to_string());

        let expected_cron = "*/20 * * * *".to_string();
        let expected_command =
            ". ~/.myenv; /usr/bin/python3 /Users/wstreet7/project/check_tasks/check_tasks.py;"
                .to_string();

        assert_eq!(cron_job.cron, expected_cron);
        assert_eq!(cron_job.command, expected_command);
        assert_eq!(cron_job.comment, Some("".to_string())); // No comment should be parsed
        assert_eq!(cron_job._disabled, false); // Not _disabled
    }

    #[test]
    fn test_empty_cron_job() {
        // Test an empty job string
        let job_string = "";

        let cron_job = CronJob::new(job_string.to_string());

        assert_eq!(cron_job.cron, "");
        assert_eq!(cron_job.command, "");
        assert_eq!(cron_job.comment, Some("".to_string()));
        assert_eq!(cron_job._disabled, false); // Empty job should not be _disabled
    }

    #[test]
    fn test_cron_job_with_only_cron_expression() {
        // Test a cron job with only the cron expression and no command
        let job_string = "*/20 * * * *";

        let cron_job = CronJob::new(job_string.to_string());

        let expected_cron = "*/20 * * * *".to_string();
        let expected_command = "".to_string(); // No command

        assert_eq!(cron_job.cron, expected_cron);
        assert_eq!(cron_job.command, expected_command);
        assert_eq!(cron_job.comment, Some("".to_string())); // No comment
        assert_eq!(cron_job._disabled, false); // Not _disabled
    }
}
