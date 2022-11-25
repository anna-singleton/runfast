extern crate skim;

use clap::Parser;
use skim::prelude::*;

mod cli;
mod runner;
mod runnercache;

use cli::Cli;
use runner::Runner;
use runnercache::RunnerCache;

fn select_new_runner(runners_path: Option<String>) -> Option<Runner> {
    let runners = runner::load_runners(&runners_path);

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

    if result.final_event == Event::EvActAbort || result.selected_items.is_empty() {
        return None
    }

    if result.selected_items.len() > 1 {
        unreachable!("Unable to process multiple items.");
    }

    let key = result.selected_items[0].output();

    let mut chosen_runner = None;
    for mut r in runners {
        if r.name == key {
            r.get_args();
            chosen_runner = Some(r);
        }
    }

    chosen_runner
}

fn main() {
    let cli = Cli::parse();

    let mut cache = RunnerCache::load();

    let chosen = if cli.force_choose_new {
        let runner = select_new_runner(cli.runners_path);
        match cache {
            Some(mut cache) => {
                if let Some(ref runner) = runner {
                    cache.add_runner(runner);
                }
            },
            None => {
                eprintln!("Couldn't parse cache, intentionally not overwriting, check it for errors.");
            }
        }
        runner
    } else {
        match cache {
            Some(ref mut cache) => match cache.try_get_runner() {
                Some(runner) => Some(runner), // runner found in the cache
                None => { // runner not found in the cache
                    let runner = select_new_runner(cli.runners_path);
                    if let Some(ref runner) = runner {
                        cache.add_runner(runner);
                    }
                    runner
                },
            },
            None => select_new_runner(cli.runners_path),
        }
    };

    match chosen {
        Some(cr) => cr.run(),
        None => println!("No Runner Selected"),
    };
}
