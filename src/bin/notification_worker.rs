use company_house_monitoring::workers::notification_worker::NotificationWorker;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let mut worker = NotificationWorker::new().await;
    worker.do_work().await;
}
