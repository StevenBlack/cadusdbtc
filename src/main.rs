use reqwest;
use std::fs::OpenOptions;
use std::io::Write;
use tokio_cron_scheduler::{Job, JobScheduler};
use chrono::Local;

#[tokio::main]
async fn main() {
    let mut sched = JobScheduler::new();

    // sec   min   hour   day of month   month   day of week
    // *     *     *      *              *       *
    let _ = sched.add(Job::new_async("* */30 * * * *", |_uuid, _l| {
        Box::pin(async {
            fetch_and_log().await;
        })
    }).unwrap());

    sched.start().await.unwrap();
}

async fn fetch_and_log() {
    let url = "https://min-api.cryptocompare.com/data/price?fsym=BTC&tsyms=cad,usd";
    match reqwest::get(url).await {
        Ok(response) => {
            if let Ok(json) = response.text().await {
                let now = Local::now();
                let log_entry = format!("{} - {}\n", now.to_rfc3339(), json);
                println!("{}", log_entry);
                let mut file = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open("log.txt")
                    .unwrap();
                file.write_all(log_entry.as_bytes()).unwrap();
            }
        }
        Err(e) => eprintln!("Error fetching data: {}", e),
    }
}
