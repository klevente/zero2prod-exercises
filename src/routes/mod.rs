mod health_check;
mod subscriptions;
mod subscriptions_confirm;

pub use health_check::*;
pub use subscriptions::*;
pub use subscriptions_confirm::*;

/// Traverse the whole error chain to the root cause, printing out everything on the way
/// This feature is available in nightly by using `std::error::Error::chain`
pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}
