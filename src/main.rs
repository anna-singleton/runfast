extern crate skim;
use directories::BaseDirs;
use skim::prelude::*;
use std::path::PathBuf;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

mod runner;
use runner::Runner;

#[derive(Serialize, Deserialize, Debug)]
struct RunnerCache {
    runners: HashMap<PathBuf, Runner>,
}

impl RunnerCache {
    fn load() -> Option<RunnerCache> {
        let cache_path = BaseDirs::new()
            .unwrap()
            .cache_dir()
            .join("runfast-cache.toml");

        if !cache_path.exists() {
            return Some(RunnerCache { runners: HashMap::new() })
        }

        let cache_string = std::fs::read_to_string(cache_path).unwrap();

        return match toml::from_str::<RunnerCache>(&cache_string) {
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

    fn try_get_runner(&self) -> Option<Runner> {
        match self.runners.get(&std::env::current_dir().unwrap()) {
            Some(rnr) => Some(rnr.to_owned()),
            None => None,
        }
    }

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

pub fn main() {
    let mut cache = RunnerCache::load();

    let chosen_runner = match cache {
        Some(ref mut c) => match c.try_get_runner() {
            Some(rnr) => Some(rnr),
            None => {
                let rnr = select_new_runner();
                if rnr.is_some() {
                    c.add_runner(&rnr.as_ref().unwrap());
                }
                rnr
            },
        },
        None => select_new_runner(),
    };

    match chosen_runner {
        Some(cr) => cr.run(),
        None => println!("No Runner Selected"),
    };

    println!("bye!");

}

fn select_new_runner() -> Option<Runner> {
    let runners = runner::load_runners();

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

    if result.final_event == Event::EvActAbort {
        println!("Nothing Selected...");
        return None
    }

    if result.selected_items.len() != 1 {
        unreachable!()
    }

    let key = result.selected_items[0].output();
    println!("Selected: {}", key);

    let mut chosen_runner = None;
    for r in runners {
        if r.name == key {
            chosen_runner = Some(r);
        }
    }

    chosen_runner
}
