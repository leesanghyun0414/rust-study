use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver};
use std::thread::{spawn, JoinHandle};

/// Start a thread that loads documents from the filesystem into memory.
///
/// `documents` is a list of filenames to load.
///
/// This returns a pair of values: a receiver that receives the documents, as
/// Strings; and a `JoinHandle` that can be used to wait for this thread to
/// exit and to get the `io::Error` value if anything goes wrong.
fn start_file_reader_thread(
    documents: Vec<PathBuf>,
) -> (Receiver<String>, JoinHandle<io::Result<()>>) {
    let (sender, receiver) = channel();

    let handle = spawn(move || {
        for filename in documents {
            let mut f: File = File::open(filename).unwrap();
            let mut text = String::new();
            f.read_to_string(&mut text).unwrap();
            if sender.send(text).is_err() {
                break;
            }
        }
        Ok(())
    });
    (receiver, handle)
}
