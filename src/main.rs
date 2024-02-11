use std::fmt::{Debug, Display};
use tokio::task::JoinError;
use zero2prod::{
    configuration::get_configuration,
    issue_delivery_worker::run_worker_until_stopped,
    startup::Application,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");

    let application = Application::build(configuration.clone()).await?;
    let application_task = tokio::spawn(application.run_until_stopped());
    let worker_task = tokio::spawn(run_worker_until_stopped(configuration));

    // Returns as soon as one of the tasks completes or errors out.
    // Use `select` on the `JoinHandle`s of the tasks, so they can be run in parallel.
    // If the `Future`s were passed into `select` directly, then they would run on the same thread,
    // meaning they could not be executed in parallel (hence the use of `tokio::spawn`).
    tokio::select! {
        o = application_task => report_exit("API", o),
        o = worker_task => report_exit("Background worker", o),
    }

    Ok(())
}

// - Outer `Result` for `outcome`: Whether there were issues with the task itself
// - Inner `Result` for `outcome`: Whether any error occurred inside the offloaded `Future`
fn report_exit(task_name: &str, outcome: Result<Result<(), impl Debug + Display>, JoinError>) {
    match outcome {
        Ok(Ok(())) => {
            tracing::info!("{} has exited", task_name)
        }
        Ok(Err(e)) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{} failed",
                task_name
            )
        }
        Err(e) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{}' task failed to complete",
                task_name
            )
        }
    }
}
