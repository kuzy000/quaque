use clap::Parser;
use librespot::core::authentication::Credentials;
use librespot::core::cache::Cache;
use librespot::core::config::SessionConfig;
use librespot::core::session::{Session, SessionError};
use librespot::core::spotify_id::{SpotifyId, SpotifyIdError};
use librespot::playback::audio_backend;
use librespot::playback::config::{AudioFormat, PlayerConfig};
use librespot::playback::player::Player;
use std::fmt;

/// Downloads a track from Spotify and writes it to stdout
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Spotify username
    #[clap(env)]
    username: String,

    /// Spotify password
    #[clap(env)]
    password: String,

    /// Spotify track id to download
    track_id: String,
}

enum Error {
    SpotifyIdError(SpotifyIdError),
    SessionError(SessionError),
    CacheError(std::io::Error),
}

impl From<SpotifyIdError> for Error {
    fn from(e : SpotifyIdError) -> Self {
        Error::SpotifyIdError(e)
    }
}

impl From<SessionError> for Error {
    fn from(e : SessionError) -> Self {
        Error::SessionError(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e : std::io::Error) -> Self {
        Error::CacheError(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::SpotifyIdError(_) => write!(f, "Can't parse <track-id>"),
            Error::SessionError(e) => write!(f, "{}", e),
            Error::CacheError(e) => write!(f, "Can't create librespot cache: {}", e),
        }
    }
}

type Result<T> = std::result::Result<T, Error>;

async fn try_main() -> Result<()> {
    let cli = Cli::parse();

    // TODO provide a way to specify the location
    let cache = Cache::new(Some("librespot_cache"), None, None)?;
    let credentials = cache.credentials()
        .unwrap_or_else(|| Credentials::with_password(cli.username, cli.password));

    let track = SpotifyId::from_base62(&cli.track_id)?;

    let session = Session::connect(SessionConfig::default(), credentials, Some(cache)).await?;

    let backend = audio_backend::find(Some("pipe".to_string())).unwrap();

    let player_config = PlayerConfig {passthrough: true, ..PlayerConfig::default()};
    let (mut player, _) = Player::new(player_config, session, None, move || {
        backend(None, AudioFormat::default())
    });

    player.load(track, true, 0);
    player.await_end_of_track().await;

    Ok(())
}

#[tokio::main]
async fn main() {
    match try_main().await {
        Ok(()) => (),
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}