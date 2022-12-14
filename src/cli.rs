use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
pub(crate) struct Cli {
    #[arg(short, long="force-choose", help="Force runfast to choose a new runner, instead of \
        looking for one that may already be set")]
    pub(crate) force_choose_new: bool,
    #[arg(short, long="runners-path", help="Load specific toml config")]
    pub(crate) runners_path: Option<String>,
    #[arg(short='c', long="clean-cache", help="Remove cached directories that no longer exist")]
    pub(crate) clean_cache: bool,
    #[arg(long="reset-cache", help="Remove ALL directories in the cache")]
    pub(crate) reset_cache: bool,
}
