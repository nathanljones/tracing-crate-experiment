use std::fs::File;
use std::sync::Mutex;
use tracing::{info, trace, warn, error, instrument};
use tracing_subscriber::FmtSubscriber;

fn main() {

    let log_file = File::create("my_cool_trace.log").unwrap();
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .with_writer(Mutex::new(log_file))
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
    let number = 5;
    info!("The number is {}", number);

    let result = compute(number);
    info!("The result is {}", result);

    let number = 15;
    info!("The number is {}", number);

    let result = compute(number);
    info!("The result is {}", result);


}
#[instrument]
fn compute(n: i32) -> i32 {
    trace!("Computing the value...");
    if n > 10 {
        warn!("The number is greater than 10");
    } else if n < 1 {
        error!("The number is less than 1");
    }
    n * 2
}