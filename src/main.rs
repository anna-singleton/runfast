extern crate skim;
use directories::BaseDirs;
use skim::prelude::*;
use std::path::PathBuf;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

mod runner;
use runner::Runner;

#[derive(Serialize, Deserialize)]
struct RunnerCache {
    runners: HashMap<PathBuf, Runner>,
}

pub fn main() {
    let runners = runner::load_runners();
    let chosen_runner = select_new_runner(runners);
    match chosen_runner {
        Some(cr) => cr.run(),
        None => println!("No Runner Selected"),
    };

    println!("Current Dir: {:?}", std::env::current_dir());

    println!("bye!");

}

fn get_chosen_runner() -> Option<Runner> {
    let cache_path = BaseDirs::new()
        .unwrap()
        .cache_dir()
        .join("runfast-cache.toml");

    if !cache_path.exists() {
        return None
    }

    let cache_string = std::fs::read_to_string(cache_path).unwrap();

    let current_dir = std::env::current_dir().unwrap();

    match toml::from_str::<RunnerCache>(&cache_string) {
        Ok(cache) => return match cache.runners.get(&current_dir) {
            Some(rnr) => Some(rnr.to_owned()),
            None => None,
        },
        Err(e) => {
            println!("Could Not Parse Cache with Error: {}\n\
                Continuing without cache use.", e);
            return None;
        },
    }
}

fn select_new_runner(runners: Vec<Runner>) -> Option<Runner> {
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
