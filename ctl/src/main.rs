use clap::{AppSettings, Parser, Subcommand};

#[derive(Subcommand)]
#[clap(global_setting = AppSettings::DeriveDisplayOrder)]
enum Commands {
    /// Print current daemon status
    Status {},

    /// Start playing track using given <track_uri>
    Start {
        track_uri: String,
        db_id: String,
    },

    /// Stop playing the current track
    Stop {},

    /// Pauses the currently playing track
    Pause {},

    /// Resume the currently playing track
    Resume {},

    /// Start playing the next track in the queue
    Next {},

    /// Start playing the previous track in the queue
    Prev {},

    /// Seeks to the specified position
    Seek {},
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

fn main() {
    let _cli = Cli::parse();
}