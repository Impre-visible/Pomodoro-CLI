# Pomodoro-CLI
A simple CLI tool to manage and start pomodoro sessions

## Installation

### For Ubuntu/Debian
 - Download the .deb file from the [releases](https://github.com/Impre-visible/Pomodoro-CLI/releases) page
 - Run `sudo dpkg -i pomodoro-cli_x.x.x-1_amd64.deb` (replace x.x.x with the version you downloaded)
 - Run `sudo apt install -f` to install the dependencies if needed
 - Run `pomodoro-cli` to start the program

### For Arch based distros
 - Download the .pkg.tar.zst file from the [releases](https://github.com/Impre-visible/Pomodoro-CLI/releases) page
 - Run `sudo pacman -U pomodoro-cli-x.x.x-1-x86_64.pkg.tar.zst` (replace x.x.x with the version you downloaded)
 - Run `pomodoro-cli` to start the program

### For any other distro
 - Download or clone the repository
 - Make sure to have rust and cargo installed
 - Run `cargo build --release` in the root of the repository
 - Run `./target/release/pomodoro-cli` to start the program

## Usage
To start a pomodoro session, simply run:

```sh
pomodoro-cli
```

## Configuration

The configuration file has a different location depending on the OS:

- Linux: `~/.config/pomodoro-cli/config.toml`
- Windows: `C:\Users\<username>\AppData\Roaming\imprevisible\pomodoro-cli\config.toml`
- MacOS: `/Users/<username>/Library/Application Support/imprevisible/pomodoro-cli/config.toml`

The configuration file is created automatically when you run the program for the first time.

The default configuration is:

```toml
pomodoro_cycles = 4
work_duration = 25
short_break_duration = 5
long_break_duration = 20
```

You can change the values to your liking.

## Features

- Start a pomodoro session
- Receive desktop notifications when a session starts, ends, or when a break starts
- Change the duration of the work session, short break, and long break
- Change the number of cycles before a long break

## To-Do

- [ ] Add a pause feature (when pressing p for example)
- [ ] Add a stop feature (when pressing q for example)
- [ ] Add a way to add a notification sound (maybe a flag in the configuration file to force the sound, even if the system is muted)

## Built With
- [Rust](https://www.rust-lang.org/) - The programming language used
- [Serde](https://serde.rs/) - For serialization and deserialization
- [Directories](https://docs.rs/directories/6.0.0/directories/) - For handling platform-specific directories
- [TOML](https://github.com/toml-lang/toml) - For configuration file parsing
- [notify-rust](https://docs.rs/notify-rust/4.11.4/notify_rust/) - For desktop notifications

## License

This project is licensed under the CC BY-SA 4.0 License - see the [LICENSE](LICENSE) file for details.


## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.
