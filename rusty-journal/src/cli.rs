use std::path::PathBuf;
use structopt::StructOpt;
#[derive(Debug, StructOpt)]
pub enum Action {
    /// Write task to the journal
    Add {
        /// The task description text.
        #[structopt()]
        text: String,
    },
    /// Remove an entry from the journal file by position.
    Done {
        #[structopt()]
        position: usize,
    },
    /// List all the tasks in the journal file.
    List,

}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Rusty Journal",
    about = "A command line to-do App written in Rust."
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    /// Use different journal file.
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}