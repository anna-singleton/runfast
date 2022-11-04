extern crate skim;
use clap::Parser;
use directories::BaseDirs;
use skim::prelude::*;

use directories::BaseDirs;

use std::path::PathBuf;
use std::collections::HashMap;

use serde::{Serialize, Deserialize};

mod cli;
mod runner;

use cli::Cli;
use runner::Runner;

#[derive(Serialize, Deserialize, Debug)]
struct RunnerCache {
    runners: HashMap<PathBuf, Runner>,
}

impl RunnerCache {
    /// Returns the cache if its a valid cache, and the executing user has
    /// access to the cache
    fn load() -> Option<RunnerCache> {
        let cache_path = BaseDirs::new()
            .unwrap()
            .cache_dir()
            .join("runfast-cache.toml");

        if !cache_path.exists() {
            return Some(RunnerCache { runners: HashMap::new() })
        }

        let cache_string = std::fs::read_to_string(cache_path).unwrap();

        match toml::from_str::<RunnerCache>(&cache_string) {
            Ok(cache) => Some(cache),
            Err(e) => {
                println!("Could Not Parse Cache with Error: {}\n\
                    Continuing without cache use.", e);
                None
                // return none to signify intentionally not generating a blank
                // cache, otherwise we may overwrite an existing one that has
                // parse errors
            },
        }
    }

    /// Returns a Some(Runner) if the path exists in the cache, or None if it
    /// does not
    fn try_get_runner(&self) -> Option<Runner> {
        let cdir = std::env::current_dir().unwrap();
        self.runners.get(&cdir).map(|rnr| rnr.to_owned())
    }

    /// Adds a runner to the cache, serialises it, then writes it to disk.
    ///
    /// In the case the current filepath is already in the cache, overwrite it
    /// with the new value of the runner
    ///
    /// # Arguments:
    ///
    /// * `runner` - A borrowed runner to be added to the cache.
    ///
    fn add_runner(&mut self, runner: &Runner) {
        let current_path = std::env::current_dir().unwrap();
        if self.runners.contains_key(&current_path) {
            self.runners.remove(&current_path);
        }
        self.runners.insert(current_path, runner.clone());

        let new_cache = match toml::to_string(&self) {
            Ok(nc) => nc,
            Err(e) => {
                println!("Could not serialise new cache data to toml, error: {}", e);
                return;
            },
        };

        let cache_path = BaseDirs::new()
            .unwrap()
            .cache_dir()
            .join("runfast-cache.toml");

        match std::fs::write(cache_path, new_cache) {
            Ok(_) => (),
            Err(e) => println!("Could not write toml to disk, error: {}", e),
        };
    }
}

fn select_new_runner(
    path: String,
) -> Option<Runner> {
    let runners = runner::load_runners(&path);

    let options = SkimOptionsBuilder::default()
        .preview(Some(""))
        .preview_window(Some(""))
        .build()
        .unwrap();

    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

    for r in &runners {
        tx.send(Arc::new(r.clone())).unwrap();
    }

    drop(tx);

    let r = Skim::run_with(&options, Some(rx));

    if r.is_none() {
        println!("internal runquick error :(");
        return None
    }

    let result = r.unwrap();

    if result.final_event == Event::EvActAbort || result.selected_items.len() == 0 {
        return None
    }

    if result.selected_items.len() > 1 {
        unreachable!("Unable to process multiple items.");
    }

    let key = result.selected_items[0].output();

    let mut chosen_runner = None;
    for r in runners {
        if r.name == key {
            chosen_runner = Some(r);
        }
    }

    chosen_runner
}

fn main() {
    let cli = Cli::parse();

    let mut cache = RunnerCache::load();

    let chosen = if cli.force_choose_new {
        let runner = select_new_runner(cli.config_path);
        match cache {
            Some(mut cache) => {
                if runner.is_some() {
                    cache.add_runner(&runner.as_ref().unwrap());
                }
            },
            None => {
                eprintln!("Couldn't parse cache, intentionally not overwriting, check it for errors.");
                std::process::exit(1);
            }
        }
        runner
    } else {
        match cache {
            Some(ref mut cache) => match cache.try_get_runner() {
                Some(runner) => Some(runner), // runner found in the cache
                None => { // runner not found in the cache
                    let runner = select_new_runner(cli.config_path);
                    if runner.is_some() {
                        cache.add_runner(&runner.as_ref().unwrap());
                    }
                    runner
                },
            },
            None => select_new_runner(cli.config_path),
        }
    };

    match chosen {
        Some(cr) => cr.run(),
        None => println!("No Runner Selected"),
    };
}
