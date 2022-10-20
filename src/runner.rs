use std::borrow::Cow;

use skim::*;

#[derive(Debug, Clone)]
pub struct Runner {
    name: String,
    options: String
}

impl Runner {
    pub fn new(name: &str, options: &str) -> Runner {
        Runner {
            name: name.to_string(),
            options: options.to_string()
        }
    }
}

impl SkimItem for Runner {
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
