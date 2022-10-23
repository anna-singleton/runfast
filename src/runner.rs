use std::borrow::Cow;
use std::fs::File;
use std::fs::read_to_string;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use directories::BaseDirs;

use skim::*;
use serde_derive::Deserialize;

#[derive(Debug, Clone)]
pub struct Runner {
    pub name: String,
    pub cmd: String,
    pub quit_fast: bool,
}

impl Runner {
    pub fn new(name: &str, cmd: &str, close_fast: bool) -> Runner {
        Runner {
            name: name.to_string(),
            cmd: cmd.to_string(),
            quit_fast: close_fast
        }
    }

    pub fn new_from_raw(_: &RunnerRaw) -> Runner {
        //TODO: likely using the toml crate, implement parsing a toml block as
        //a runner, given the whole thing as a string
        todo!()
    }

    pub fn run(&self) {
        let mut c = Command::new("bash");
        c.arg("-c");
        c.arg(&self.cmd);
        let result = c.status();
        if result.is_err() {
            println!("Error Running Command: {:#?}", result);
        }
        if !self.quit_fast {
            println!("Press ENTER to exit...");
            let _ = Command::new("bash").arg("-c").arg("read").status();
        }
    }
}


#[derive(Debug, Deserialize)]
struct Config {
    defaults: Option<RunnerConfig>,
    runners: Option<Vec<RunnerConfig>>,
}

// Struct to use for parsing toml, since each runner in the toml may not have
// a complete config defined, but we can construct one out of RUNNER_DEFAULTS
// and the defaults in both config files
#[derive(Debug, Deserialize)]
struct RunnerConfig {
    name: Option<String>,
    cmd: Option<String>,
    quit_fast: Option<bool>,
}

// defaults to use if [defaults] is not defined, or incompletely defined in
// the config file(s)
const RUNNER_DEFAULTS: RunnerConfig = RunnerConfig {
    name: None,
    cmd: None,
    quit_fast: Some(false),
};

pub fn load_runners() {
    // try to load ~/.config/runfast/defaults.toml and ~/.config/runfast/runners.toml
    // prefer values in runners.toml if there are clashes
    if let Some(base_dirs) = BaseDirs::new() {
        // get default config directory (usually ~/.config/)
        let confdir = base_dirs.config_dir();

        // load default config
        let default_path = confdir.join("runfast/defaults.toml");
        let mut default_configs: Option<Config> = None;
        if !default_path.exists() {
            generate_default_config(&default_path);
            let default_confstring = read_to_string(default_path).unwrap();
            match toml::from_str::<Config>(&default_confstring) {
                Ok(conf) => default_configs = Some(conf),
                Err(e) => panic!("Could not parse default config: {}", e),
            }
        }
        println!("Default Config: {:#?}", default_configs);

        // load user config
        let userconf_path = confdir.join("runfast/runners.toml");
        let mut user_configs: Option<Config> = None;
        if userconf_path.exists() {
            let user_confstring = read_to_string(userconf_path).unwrap();
            match toml::from_str::<Config>(&user_confstring) {
                Ok(conf) => user_configs = Some(conf),
                Err(e) => panic!("Could not parse user config: {}", e),
            }
        }
        println!("User Config: {:#?}", user_configs);
    }
}

fn generate_default_config(default_path: &Path) {
    let default_conf = File::create(default_path);
    match default_conf {
        Ok(mut conf_file) => {
            conf_file.write(
            b"[defaults]\n\
            name=\"default name\"\n\
            cmd=\"echo no command set\"\n\
            quit_fast=false\n\n\
            [[runners]]\n\
            name=\"rust run\"\n\
            cmd=\"cargo run\"\n\
            quit_fast=false").unwrap();
            ()
        },
        Err(e) => {
            println!("Could not create file at: {}, error: {:#?}",
                default_path.display(), e);
            panic!("No default config could be created, panicing");
        },
    }
}





#[derive(Debug, Clone)]
pub struct RunnerRaw {
    name: String,
    options: String
}

impl RunnerRaw {
    pub fn new(name: &str, options: &str) -> RunnerRaw {
        RunnerRaw {
            name: name.to_string(),
            options: options.to_string()
        }
    }
}

impl SkimItem for RunnerRaw {
    fn text(&self) -> prelude::Cow<str> {
        Cow::Borrowed(&self.name)
    }

    fn preview(&self, _context: PreviewContext) -> ItemPreview {
        let mut prev = String::new();
        prev.push_str("[");
        prev.push_str(self.name.as_str());
        prev.push_str("]\n");
        prev.push_str(self.options.as_str());
        ItemPreview::Text(prev)
    }
}
