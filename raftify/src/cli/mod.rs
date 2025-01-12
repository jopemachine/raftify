include!(concat!(env!("OUT_DIR"), "/built.rs"));

mod commands;

use clap::{Args, Parser, Subcommand};
use commands::{
    debug::{debug_entries, debug_node, debug_persisted, debug_persitsted_all},
    dump::dump_peers,
    utils::parse_peers_json,
};
use std::fmt::Debug;

use crate::{
    raft::{default_logger, formatter::set_custom_formatter},
    AbstractLogEntry, AbstractStateMachine, CustomFormatter, Result,
};

#[derive(Parser)]
#[command(name = "raftify")]
#[command(version = PKG_VERSION)]
#[command(author = PKG_AUTHORS)]
#[command(about = PKG_DESCRIPTION)]
struct App {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Debug tools
    #[command(subcommand)]
    Debug(DebugSubcommands),
    /// Dump tools
    Dump(Dump),
}

#[derive(Subcommand)]
enum DebugSubcommands {
    /// List persisted log entries and metadata
    Persisted {
        /// The log directory path
        path: String,
    },
    /// List persisted log entries and metadata for all local nodes
    PersistedAll {
        /// The log directory path
        path: String,
    },
    /// List all log entries
    Entries {
        /// The address of the RaftNode
        address: String,
    },
    /// Inspect RaftNode
    Node {
        /// The address of the RaftNode
        address: String,
    },
}

#[derive(Args)]
struct Dump {
    /// The log directory path
    path: String,
    /// The peers to dump
    peers: String,
}

pub async fn cli_handler<
    LogEntry: AbstractLogEntry + Debug + Send + 'static,
    FSM: AbstractStateMachine + Debug + Clone + Send + Sync + 'static,
>(
    args: Option<Vec<String>>,
) -> Result<()> {
    let app: App = match args {
        Some(args) => App::parse_from(args),
        None => App::parse(),
    };
    let logger = default_logger();
    set_custom_formatter(CustomFormatter::<LogEntry, FSM>::new());

    match app.command {
        Commands::Debug(x) => match x {
            DebugSubcommands::Persisted { path } => {
                debug_persisted(path.as_str(), logger.clone())?;
            }
            DebugSubcommands::PersistedAll { path } => {
                debug_persitsted_all(path.as_str(), logger.clone())?;
            }
            DebugSubcommands::Entries { address } => {
                debug_entries(address.as_str()).await?;
            }
            DebugSubcommands::Node { address } => {
                debug_node(address.as_str()).await?;
            }
        },
        Commands::Dump(x) => {
            dump_peers(
                x.path.as_str(),
                parse_peers_json(x.peers.as_str()).unwrap(),
                logger.clone(),
            )?;
        }
    }

    Ok(())
}
