use std::borrow::Cow;
use std::process::Command;

use skim::*;

#[derive(Debug, Clone)]
pub struct Runner {
    pub name: String,
    pub cmd: String,
    pub args: Option<Vec<String>>,
    pub close_fast: bool,
}

impl Runner {
    pub fn new(name: &str, cmd: &str, args: Option<Vec<String>>, close_fast: bool) -> Runner {
        Runner {
            name: name.to_string(),
            cmd: cmd.to_string(),
            args,
            close_fast
        }
    }

    pub fn new_from_string(s: &String) -> Runner {
        //TODO: likely using the toml crate, implement parsing a toml block as
        //a runner, given the whole thing as a string
        todo!()
    }

    pub fn run(&self) {
        let mut c = Command::new(&self.cmd);
        if self.args.is_some() {
            c.args(self.args.as_ref().unwrap());
        }
        c.spawn()
         .expect("could not spawn command");
        if !self.close_fast {
            println!("Press ENTER to exit...");
            let _ = Command::new("pause").status();
        }
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
