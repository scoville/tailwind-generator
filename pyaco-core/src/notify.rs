use std::time::Duration;

use notify::RecommendedWatcher;
use notify_debouncer_mini::{new_debouncer, DebouncedEvent, Debouncer};
use tokio::{runtime::Handle, sync::mpsc};

use crate::Result;

#[allow(
    clippy::type_complexity,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc
)]
pub fn async_debounced_watcher(
    timeout: Duration,
) -> Result<(
    Debouncer<RecommendedWatcher>,
    mpsc::Receiver<std::result::Result<Vec<DebouncedEvent>, notify::Error>>,
)> {
    let (tx, rx) = mpsc::channel(1);

    let debouncer = new_debouncer(timeout, move |res| {
        Handle::current().block_on(async {
            tx.send(res).await.unwrap();
        });
    })?;

    Ok((debouncer, rx))
}
