use company_house_monitoring::workers::process_updates_worker::ProcessUpdatesWorker;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let mut worker = ProcessUpdatesWorker::new().await;
    worker.do_work().await;
}
