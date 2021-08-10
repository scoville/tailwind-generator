use anyhow::Result;
use grep_matcher::{Captures, Matcher};
use grep_regex::RegexMatcher;
use grep_searcher::{sinks::UTF8, SearcherBuilder};
use log::error;
use regex::Regex;
use std::{borrow::Borrow, collections::HashSet, fs::File, path::Path, process::exit};

pub async fn extra_classes_from_path<C, S>(
    file: File,
    capture_regex: C,
    split_regex: S,
) -> Result<HashSet<String>>
where
    C: Borrow<RegexMatcher>,
    S: Borrow<Regex>,
{
    // Classes found in the visited file
    let mut found_classes = HashSet::new();

    // TODO: We may be able to reuse the same searcher for all the files?
    let mut searcher = SearcherBuilder::new().multi_line(true).build();

    let capture_regex = capture_regex.borrow();

    searcher.search_file(
        capture_regex,
        &file,
        UTF8(|_, line| {
            let mut captures = capture_regex.new_captures()?;

            // Search for the captures pattern...
            if capture_regex.captures(line.as_bytes(), &mut captures)? {
                if let Some(m) = captures.get(1) {
                    let classes = &line[m];

                    // ... and then split the captured classes
                    for class in split_regex.borrow().split(classes) {
                        if !class.is_empty() {
                            found_classes.insert(class.to_string());
                        }
                    }
                }
            }

            Ok(true)
        }),
    )?;

    Ok(found_classes)
}

pub async fn open_file<P: AsRef<Path>>(path: P) -> Option<File> {
    match File::open(path.as_ref()) {
        Ok(file) => Some(file),
        Err(error) => {
            if let Some(24) = error.raw_os_error() {
                eprintln!("Too many files opened [os error 24], please use the --max-opened-files options to lower the amount of opened files");

                exit(2);
            }

            error!("An error occured when trying to open {:?}", path.as_ref());

            None
        }
    }
}
