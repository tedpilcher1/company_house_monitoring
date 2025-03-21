use company_house_monitoring::{
    database::client::DatabaseClient, workers::streaming_worker::StreamingWorker,
};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let mut database = DatabaseClient::new().expect("Should be able to connect to db");
    let last_timepoint = database
        .get_last_processed_timepoint()
        .expect("Should be able to get last processed timepoint");
    let mut worker = StreamingWorker::new().await;
    worker.do_work(last_timepoint).await;
}
