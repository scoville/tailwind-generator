use std::path::PathBuf;

use async_trait::async_trait;
use compact_str::CompactString;

pub mod console;

#[async_trait]
pub trait Sink {
    type Event;

    fn send(&mut self, event: Self::Event);

    async fn done(self) -> bool;
}

pub type SearchFileEvent = (u64, PathBuf, CompactString);
