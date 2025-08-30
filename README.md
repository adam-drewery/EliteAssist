# EliteAssist
_A Linux compatible Elite:Dangerous companion for your second monitor_

[![Rust](https://github.com/adam-drewery/EliteAssist/actions/workflows/pipeline.yml/badge.svg)](https://github.com/adam-drewery/EliteAssist/actions/workflows/pipeline.yml)

## Contents

- [Features](#features)
- [How to install](#how-to-install)
- [Requirements](#requirements)
- [Current limitations](#current-limitations)
- [Contributing](#contributing)
- [Thanks](#thanks)

## Features

- Fully customizable UI: Add new screens and choose which panels to show on it via the settings menu.
- Drag and drop panels to rearrange them on a screen.
- Screens and layout configurations are automatically persisted to a file.
- CTRL+Tab hotkey for switching between screens while in-game.
- Fullscreen mode.

- Currently implemented panels include:
  - Personal equipment loadout.
  - Ranks and rank progress.
  - Chat message log.
  - Game events log.
  - Ship details.
  - Ship module loadout.
  - Route details including scoopable stars.
  - Location details.
  - Engineering materials.
  - Ship locker contents.
  - Market data (in-progress).
  - Missions (in-progress).
  - Outstanding claims (in progress).

![Main application screen](docs/main_screen.png)

## How to install

Just download the latest release from [here](https://github.com/adam-drewery/EliteAssist/tags).

Remember to mark the file as executable: `chmod +x`.

## Requirements

You need [Elite:Dangerous](https://store.steampowered.com/app/359320/Elite_Dangerous/) installed via steam. The application expects the default `.steam` symlink in your home directory.

## Current limitations

- Currently the market screen only updates when you open the market in game. So... not so useful yet.
- Some journal events aren't handled yet. In particular the fleet carrier and colonization stuff.
- The UI is tested at 1920x1080 resolution and won't look so good much smaller than that.

## Contributing

Open a [pull request](https://github.com/adam-drewery/EliteAssist/pulls) or maybe start a [discussion](https://github.com/adam-drewery/EliteAssist/discussions) if you have a cool idea. If you found a bug why not [open an issue](https://github.com/adam-drewery/EliteAssist/issues).

## Thanks
Big thanks to:
- CMDR Qohen Leth for their awesome [cartoon ship vectors](https://www.reddit.com/r/EliteDangerous/comments/1mnmolv/elite_cartoon_ship_vectors_elite_ships_colouring/)
- the [INARA](https://inara.cz) team.
- Whoever made and maintains [EDSM](https://www.edsm.net).
- Iain Collins for the [Ardent](https://github.com/iaincollins/ardent-api) API.
- "jixxed" for these handy [journal schemas](https://github.com/jixxed/ed-journal-schemas)
- The [Elite Dangerous Community Developers](https://github.com/EDCD).
- The [Elite:Dangerous](https://www.elitedangerous.com) team.
- The [Rust](https://www.rust-lang.org) team.
