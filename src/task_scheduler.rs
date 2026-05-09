use std::thread;
use std::time::Duration;

pub struct TaskScheduler;

impl TaskScheduler {
    pub fn start<F>(interval: Duration, task: F)
    where
        F: Fn() + Send + 'static,
    {
        thread::spawn(move || loop {
            task();
            thread::sleep(interval);
        });
    }

    pub fn run_daily<F>(hour: u32, minute: u32, task: F)
    where
        F: Fn() + Send + 'static,
    {
        thread::spawn(move || loop {
            let now = chrono::Local::now();
            let next = now.date_naive().and_hms_opt(hour, minute, 0).unwrap();
            let wait = if next > now.naive_local() {
                next - now.naive_local()
            } else {
                (next + chrono::Duration::days(1)) - now.naive_local()
            };
            thread::sleep(wait.to_std().unwrap_or(Duration::from_secs(3600)));
            task();
        });
    }
}
