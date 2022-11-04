use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
pub(crate) struct Cli {
    #[arg(short, long="force-choose", help="Force runfast to choose a new runner, instead of \
        looking for one that may already be set")]
    pub(crate) force_choose_new: bool,
    #[arg(short, long="runners-path", help="Load specific toml config", default_value="~/.config/runfast/runners.toml")]
    pub(crate) runners_path: String,
}
