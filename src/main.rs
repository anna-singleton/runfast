extern crate skim;
use skim::prelude::*;

mod runner;
use runner::Runner;

pub fn main() {
    let runners = runner::load_runners();
    let chosen_runner = select_new_runner(runners);
    match chosen_runner {
        Some(cr) => cr.run(),
        None => println!("No Runner Selected"),
    };

    println!("bye!");

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
