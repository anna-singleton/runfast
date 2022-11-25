use std::path::PathBuf;
use std::collections::HashMap;
use directories::BaseDirs;
use serde::{Serialize, Deserialize};

use crate::runner::Runner;

#[derive(Serialize, Deserialize, Debug)]
pub struct RunnerCache {
    runners: HashMap<PathBuf, Runner>,
}

impl RunnerCache {
    /// Returns the cache if its a valid cache, and the executing user has
    /// access to the cache
    pub fn load() -> Option<RunnerCache> {
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
    pub fn try_get_runner(&self) -> Option<Runner> {
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
    /// * [runner](Runner) - A borrowed runner to be added to the cache.
    ///
    pub fn add_runner(&mut self, runner: &Runner) {
        let current_path = std::env::current_dir().unwrap();
        if self.runners.contains_key(&current_path) {
            self.runners.remove(&current_path);
        }
        self.runners.insert(current_path, runner.clone());

        let new_cache = match toml::to_string(&self) {
            Ok(nc) => nc,
            Err(e) => {
                eprintln!("Could not serialise new cache data to toml, error: {}", e);
                return;
            },
        };

        let cache_path = BaseDirs::new()
            .unwrap()
            .cache_dir()
            .join("runfast-cache.toml");

        match std::fs::write(cache_path, new_cache) {
            Ok(_) => (),
            Err(e) => eprintln!("Could not write toml to disk, error: {}", e),
        };
    }
}
