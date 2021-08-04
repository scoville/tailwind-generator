use anyhow::Result;
use clap::{crate_version, Clap};
use glob::glob;
use grep_matcher::{Captures, Matcher};
use grep_regex::RegexMatcher;
use grep_searcher::{sinks::UTF8, SearcherBuilder};
use log::{error, info};
use regex::Regex;
use std::{
    borrow::Borrow, collections::HashSet, fs::File, path::PathBuf, process::exit, sync::Arc,
};
use style_generator_core::{
    classify_path, extract_classes_from_file, extract_classes_from_url, InputType,
};

#[derive(Clap, Debug)]
#[clap(name = "style-generator", version = crate_version!())]
struct Options {
    /// CSS file path or URL used for code verification
    #[clap(short, long)]
    css_input: String,

    /// Glob pointing to the files to validate
    #[clap(short, long)]
    input_glob: String,

    /// Classes matcher regex, must include a capture containing all the classes
    #[clap(long, default_value = r#"class="([^"]+)""#)]
    capture_regex: String,

    /// Classes splitter regex, will split the string captured with the `capture_regex` argument and split it into classes
    #[clap(long, default_value = r#"\s+"#)]
    split_regex: String,
}

async fn extra_classes_from_path<C, S>(
    path: PathBuf,
    capture_regex: C,
    split_regex: S,
) -> Result<HashSet<String>>
where
    C: Borrow<RegexMatcher>,
    S: Borrow<Regex>,
{
    let file = File::open(path)?;

    let mut found_classes = HashSet::new();

    let mut searcher = SearcherBuilder::new().multi_line(true).build();

    let capture_regex = capture_regex.borrow();

    searcher.search_file(
        capture_regex,
        &file,
        UTF8(|_, line| {
            let mut captures = capture_regex.new_captures()?;

            if capture_regex.captures(line.as_bytes(), &mut captures)? {
                if let Some(m) = captures.get(1) {
                    let classes = &line[m];

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

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let Options {
        css_input,
        input_glob,
        capture_regex,
        split_regex,
    } = Options::parse();

    let capture_regex = Arc::new(RegexMatcher::new(capture_regex.as_str())?);

    let split_regex = Arc::new(Regex::new(split_regex.as_str())?);

    info!("Validating {} against {}", input_glob, css_input);

    let css_input = classify_path(css_input);

    let mut glob = glob(input_glob.as_str())?;

    let accepted_classes = match css_input {
        InputType::Path(path) => extract_classes_from_file(path),
        InputType::Url(url) => extract_classes_from_url(url),
    }?;

    let mut jobs = Vec::new();

    while let Some(Ok(path)) = glob.next() {
        let split_regex = split_regex.clone();

        let capture_regex = capture_regex.clone();

        let job = tokio::spawn(extra_classes_from_path(path, capture_regex, split_regex));

        jobs.push(job);
    }

    let mut found_classes = HashSet::new();

    for job in jobs {
        for class in job.await?? {
            found_classes.insert(class);
        }
    }

    let unknown_classes = found_classes
        .difference(&accepted_classes)
        .collect::<HashSet<&String>>();

    info!(
        "{} classes used in total throughout the project, {} classes are whitelisted",
        found_classes.len(),
        accepted_classes.len()
    );

    if !unknown_classes.is_empty() {
        for class in unknown_classes {
            error!("Unkown class found {}", class);
        }

        exit(1);
    }

    info!("Used classes are all valid");

    Ok(())
}
