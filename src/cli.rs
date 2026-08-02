use clap::Parser;

/// Selects how the server exchanges LSP messages with the client.
///
/// The three transport modes share `host`/`port`, but interpret them
/// differently: with neither flag set the server talks over stdio (the
/// normal case, since editors spawn the server as a subprocess); `--listen`
/// binds a TCP socket on `host`:`port` and waits for one client to connect;
/// with `--listen` absent but `host`/`port` given, the server instead
/// connects out to that address as a TCP client.
#[derive(Parser, Debug)]
#[command(version)]
pub(crate) struct Cli {
    #[arg(long)]
    pub listen: bool,
    pub host: Option<String>,
    pub port: Option<u16>,
}
