#![cfg(debug_assertions)]

use adler::adler32;
use std::fs;
use std::io::BufReader;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug)]
pub struct FileTracker {
    pub templates_hash: AtomicU64,
    pub articles_hash: AtomicU64,
}

impl FileTracker {
    pub fn new() -> Self {
        let templates_hash = AtomicU64::new(Self::hash_dir("templates"));
        let articles_hash = AtomicU64::new(Self::hash_dir("articles/md"));

        Self {
            templates_hash,
            articles_hash,
        }
    }

    fn hash_dir(path: impl AsRef<Path>) -> u64 {
        let mut h = 0;
        for entry in fs::read_dir(path).unwrap() {
            h += adler32(BufReader::new(
                fs::File::open(entry.unwrap().path()).unwrap(),
            ))
            .unwrap() as u64;
        }

        h
    }

    pub fn check_articles(&self) -> bool {
        let new = Self::hash_dir("articles/md");

        if new != self.articles_hash.load(Ordering::Relaxed) {
            self.articles_hash.store(new, Ordering::Relaxed);

            true
        } else {
            false
        }
    }

    pub fn check_templates(&self) -> bool {
        let new = Self::hash_dir("templates");

        if new != self.templates_hash.load(Ordering::Relaxed) {
            self.templates_hash.store(new, Ordering::Relaxed);

            info!("Detected changes in template, restarting process");
            // For windows development, bind mounts in docker don't send INotify
            // events. `rocket` relies on the `notify` crate, which tracks INotify events.
            // So it can't auto reload templates. Instead, just restart the process to reload templates.
            // Rocket doesn't give us access to the current Handlebars instance, so we can't reload them manually.
            std::process::exit(1);
        } else {
            false
        }
    }
}
