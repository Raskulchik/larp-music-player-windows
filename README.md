# LARP Music Player — Windows

Baka
<img width="498" height="390" alt="anime-tsundere" src="https://github.com/user-attachments/assets/522907f3-1cf0-4a72-a01c-490e553f142c" />

Windows port of [larp-music-player](https://github.com/Raskulchik/larp-music-player).
The original Linux version stays in its own repository; this one is maintained
separately so Windows changes do not affect it.

The player is a terminal UI for Yandex Music, iTunes, SoundCloud, YouTube Music
and local files, with search, playlists, liked tracks, lyrics, listening
statistics, Discord Rich Presence and a CLI.

## Requirements

- Windows 10 or newer (Windows 11 recommended)
- `ffmpeg` — required for audio decoding
- `yt-dlp` — required for SoundCloud and YouTube Music playback/downloads
- Python 3 with `ytmusicapi` — optional, improves YouTube Music metadata
- `curl.exe` — included with Windows 10/11

Install with winget:

```powershell
winget install --id Gyan.FFmpeg -e
winget install --id yt-dlp.yt-dlp -e
winget install --id Python.Python.3.13 -e
python -m pip install ytmusicapi
```

Make sure `ffmpeg`, `ffprobe`, `yt-dlp` and `python` are available in `PATH`.
Restart the terminal after installing them.

## Run a release build

1. Download `larp-music-player-windows-x86_64.zip` from the
   [latest release](https://github.com/Raskulchik/larp-music-player-windows/releases/latest).
2. Extract it. Keep `larp-music-player.exe` and `ytmeta.py` in the same folder.
3. Run `larp-music-player.exe` without arguments to open the TUI.

## Build from source

Install the Rust MSVC toolchain from <https://rustup.rs>, then:

```powershell
git clone https://github.com/Raskulchik/larp-music-player-windows.git
cd larp-music-player-windows
cargo build --release --locked
.\target\release\larp-music-player.exe
```

Run the tests with:

```powershell
cargo test --release --locked
```

The CI workflow builds and tests on `windows-latest`.

## CLI

Run the binary with arguments for CLI mode; run it without arguments to open
the TUI.

```powershell
larp-music-player.exe --help
larp-music-player.exe status
larp-music-player.exe play "Kino - Gruppa Krovi"
larp-music-player.exe volume 40
larp-music-player.exe search "Molchat Doma" -s ytmusic -n 5
larp-music-player.exe stats --top 10
```

Playback commands (`status`, `play`, `pause`, `next`, `volume`, ...) talk to a
running TUI. Library commands (`search`, `liked`, `playlists`, `playlist`,
`stats`, `local`, `info`, `config`, `sources`) work without a running player.

On Windows the TUI listens on a random `127.0.0.1` port and writes the port plus
an unguessable token to `%LOCALAPPDATA%\music-player-tui\player-control.json`.
The CLI uses that file automatically. The endpoint is not exposed to the
network.

Use `larp-music-player.exe --help` for the full command list and examples.

## Files and configuration

| What | Location |
| --- | --- |
| Config, liked tracks, playlists | `%APPDATA%\music-player-tui\` |
| Downloads, PCM cache, control file | `%LOCALAPPDATA%\music-player-tui\` |
| Log | `%TEMP%\music-player-tui.log` |

Yandex Music needs an OAuth token in the config (the TUI prompts for it).
SoundCloud works without a client ID by extracting one automatically. A custom
SoundCloud client ID, Discord application ID and Last.fm API key can also be
set in `config.json`.

## Differences from the Linux version

- MPRIS is not available on Windows.
- The Bluetooth audio-device indicator is Linux-only.
- CLI control uses an authenticated loopback socket instead of a Unix socket.
- Data and config follow Windows conventions (`%APPDATA%`, `%LOCALAPPDATA%`).
- Release artifacts are Windows `.zip` archives.

Discord Rich Presence, mouse input, Unicode titles and all music sources work
on Windows.
