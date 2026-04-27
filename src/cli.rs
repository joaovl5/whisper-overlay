use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand, Clone)]
pub enum Command {
    WaybarStatus {
        #[clap(flatten)]
        connection_opts: ConnectionOpts,
    },
    Overlay {
        #[clap(flatten)]
        connection_opts: ConnectionOpts,

        /// An optional stylesheet for the overlay, which replaces the internal style.
        #[arg(short, short, long, default_value=None)]
        style: Option<PathBuf>,

        /// Specifies the hotkey to activate voice input. You can use any
        /// key or button name from [evdev::Key](https://docs.rs/evdev/latest/evdev/struct.Key.html)
        #[arg(long, default_value = "KEY_RIGHTCTRL")]
        hotkey: String,

        /// Select an input device/source by exact CPAL device name.
        #[arg(long)]
        input_device: Option<String>,

        /// List available CPAL input devices/sources and exit.
        #[arg(long)]
        list_input_devices: bool,
    },
}

#[derive(Debug, Args, Clone)]
pub struct ConnectionOpts {
    /// The address of the the whisper streaming instance (host:port)
    #[clap(short, long, default_value = "localhost:7007")]
    pub address: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn overlay_accepts_input_device_name() {
        let cli = Cli::try_parse_from([
            "whisper-overlay",
            "overlay",
            "--input-device",
            "Easy Effects Source",
        ])
        .expect("overlay should parse an input device name");

        let Command::Overlay { input_device, .. } = cli.command else {
            panic!("expected overlay command");
        };

        assert_eq!(input_device, Some("Easy Effects Source".to_string()));
    }

    #[test]
    fn overlay_accepts_list_input_devices() {
        let cli = Cli::try_parse_from(["whisper-overlay", "overlay", "--list-input-devices"])
            .expect("overlay should parse list input devices flag");

        let Command::Overlay {
            list_input_devices, ..
        } = cli.command
        else {
            panic!("expected overlay command");
        };

        assert!(list_input_devices);
    }
}
