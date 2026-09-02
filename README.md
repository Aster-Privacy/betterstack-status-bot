<img width="200" alt="Aster" src="https://raw.githubusercontent.com/Aster-Privacy/.github/main/profile/aster_logo.png" />

# Better Stack Status Bot

[![Build](https://github.com/Aster-Privacy/Betterstack-status-bot/actions/workflows/ci.yml/badge.svg)](https://github.com/Aster-Privacy/Betterstack-status-bot/actions/workflows/ci.yml)
[![License: Unlicense](https://img.shields.io/badge/license-Unlicense-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2024%20edition-orange.svg)](https://www.rust-lang.org)

Better Stack Status Bot connects a [Better Stack](https://betterstack.com) status page to Discord. It announces every new incident and status update in the channel you choose, and it answers a `/status` command with the live uptime of each service on your page.

The bot is a single Rust binary with a small SQLite file next to it. It needs no web server, no database server, and no hosting account beyond somewhere to run a process.

Aster Privacy built it to keep the [Aster Mail](https://astermail.org) community informed, and it works with any Better Stack status page.

## What it does

- **Announces incidents.** The bot reads your status page RSS feed once a minute, posts each new entry in your updates channel, and adds a button that links back to the status page. It remembers what it has already posted, so a restart never repeats an announcement.
- **Mentions a role.** Each announcement can ping a role you nominate, so people opt in to status pings instead of receiving every one.
- **Reports live status.** The `/status` command reads the Better Stack API and replies with an embed listing every service, its current state, and its uptime percentage. The reply is ephemeral, so it stays visible only to the person who ran it.

## Commands

| Command | Who can run it | What it does |
|---|---|---|
| `/status` | Anyone in the server | Shows every service on the status page with its state and uptime |
| `~register_commands` | Application owner and anyone in `BOT_OWNERS` | Registers the slash commands with Discord. Run this once after you invite the bot |

`~` is the default prefix for text commands. To change it, set `COMMAND_PREFIX`.

## Requirements

- Rust 1.95 or later, or Docker
- A Discord application and bot token
- A Better Stack status page, an Uptime API token, and the status page ID

## Getting started

### 1. Create the Discord bot

1. Open the [Discord Developer Portal](https://discord.com/developers/applications) and select **New Application**.
2. Go to **Bot** and select **Reset Token** to reveal a token. Copy it, because Discord shows it only once.
3. On the same page, turn on **Message Content Intent** under **Privileged Gateway Intents**. The bot needs it for the `~register_commands` text command.
4. Go to **Installation**, choose a guild install context, and give the bot the `bot` and `applications.commands` scopes with the **Send Messages** and **Embed Links** permissions.
5. Open the generated install link and add the bot to your server.

### 2. Collect your Better Stack details

1. Go to [Better Stack API tokens](https://betterstack.com/settings/api-tokens), select your team, and copy a token from the **Uptime API tokens** section or create one.
2. Open your status page in the Better Stack dashboard. The URL ends in the status page ID, as in `https://uptime.betterstack.com/status-pages/123456`.
3. Note the public address of the page, such as `https://status.example.com/`. The bot reads `feed.rss` from that address, which Better Stack publishes for every status page.

### 3. Configure the bot

Copy the sample configuration and fill it in:

```
cp .env.example .env
```

To find a Discord channel, role, or user ID, turn on **Developer Mode** in Discord under **Settings**, then **Advanced**. Right-click the channel, role, or user and select **Copy ID**.

| Variable | Required | Default | Description |
|---|---|---|---|
| `DISCORD_TOKEN` | Yes | | Bot token from the Discord Developer Portal |
| `API_TOKEN` | Yes | | Better Stack Uptime API token |
| `STATUS_PAGE_ID` | Yes | | Numeric ID of your Better Stack status page |
| `STATUS_PAGE_URL` | Yes | | Public address of the status page, such as `https://status.example.com/` |
| `UPDATES_CHANNEL_ID` | No | | Channel that receives incident announcements. Without it, the bot serves `/status` only |
| `UPDATE_ROLE_ID` | No | | Role to mention in each announcement. Without it, announcements mention nobody |
| `BOT_OWNERS` | No | | Comma separated user IDs that get owner commands. The Discord application owner and any team members always have them |
| `COMMAND_PREFIX` | No | `~` | Prefix for text commands |
| `POLL_INTERVAL_SECS` | No | `60` | Seconds between RSS feed checks |
| `DATABASE_URL` | No | `sqlite:status.db` | Location of the SQLite file that records posted entries |
| `RUST_LOG` | No | `info` | Log level. Use `debug` while troubleshooting |

Keep `.env` out of version control. The included `.gitignore` already excludes it.

### 4. Run it

```
cargo run --release
```

The first run creates `status.db`, records the entries already on your feed, and stays quiet about them. Only entries published after that first run get announced.

Send `~register_commands` in any channel the bot can read. The bot replies to confirm, and `/status` becomes available within a minute.

## Run with Docker

```
docker build -t betterstack-status-bot .
docker run -d --name status-bot --restart unless-stopped \
  --env-file .env \
  -e DATABASE_URL=sqlite:/data/status.db \
  -v status-bot-data:/data \
  betterstack-status-bot
```

The volume keeps the record of announced entries across restarts. Without it, the bot treats every entry on your feed as new after each recreate.

## Run as a service

To keep the bot running on a Linux server, install the binary and add a systemd unit at `/etc/systemd/system/betterstack-status-bot.service`:

```
[Unit]
Description=Better Stack Status Bot
After=network-online.target

[Service]
Type=simple
User=statusbot
WorkingDirectory=/opt/betterstack-status-bot
EnvironmentFile=/opt/betterstack-status-bot/.env
ExecStart=/opt/betterstack-status-bot/betterstack-status-bot
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

Then enable it:

```
sudo systemctl enable --now betterstack-status-bot
```

## How it works

The bot polls `STATUS_PAGE_URL` plus `feed.rss` on the interval you set. Every entry gets an identifier built from its GUID and publication date, and the bot inserts that identifier into the `guids` table in SQLite with `INSERT OR IGNORE`. An insert that changes a row means the entry is new, so the bot announces it. An insert that changes nothing means the bot has seen the entry, so it stays quiet. Announcements go out oldest first, which keeps the channel in chronological order during an incident that produces several updates at once.

On a first run, when the table is still empty, the bot records everything already on the feed without announcing any of it. That keeps your channel from filling with your incident history the first time you start the bot. On every later start it announces anything published while it was offline.

The `/status` command takes a different path. It calls the Better Stack resources endpoint for your status page and builds the embed from the response, so the numbers are current at the moment someone asks rather than cached from the last poll. The command follows the pagination links in the response, and it splits the reply across several embeds because Discord allows only 25 fields in one.

## Build from source

```
git clone https://github.com/Aster-Privacy/Betterstack-status-bot.git
cd betterstack-status-bot
cargo build --release
```

The binary lands in `target/release/`. To check the code the way CI does, run:

```
cargo clippy --all-targets -- -D warnings
cargo +nightly fmt --check
```

Formatting uses nightly-only options from `rustfmt.toml`, which is why `cargo fmt` runs on the nightly toolchain. Building and linting work on stable.

## Troubleshooting

| Symptom | Cause and fix |
|---|---|
| The bot starts and exits at once | A required variable is missing. The log names it. Check `.env` against the table above |
| `/status` never appears in Discord | Run `~register_commands` as the owner of the Discord application, or add your user ID to `BOT_OWNERS`. Global commands take up to a minute to appear |
| `~register_commands` gets no reply | Turn on **Message Content Intent** in the Developer Portal, then restart the bot |
| Nothing gets announced | Confirm `UPDATES_CHANNEL_ID` is set and the bot can send messages and embed links in that channel. Set `RUST_LOG=debug` to see each poll |
| Old incidents get announced again | The SQLite file was lost. Point `DATABASE_URL` at persistent storage, or mount a volume when you use Docker |

## Community

Join our [Discord](https://discord.gg/R4XqRUfgWZ) to share feedback, ask questions, and contribute to the privacy community. You can also find us on [X](https://x.com/AsterPrivacy) and [Reddit](https://www.reddit.com/r/AsterPrivacy).

If you have any questions or security disclosures, email us at [hello@astermail.org](mailto:hello@astermail.org) or [security@astermail.org](mailto:security@astermail.org). **Do not open a public issue for security vulnerabilities.** Read [SECURITY.md](SECURITY.md) for the full security vulnerability disclosure process.

## Contributing

Issues and pull requests are welcome. Keep changes focused, run `cargo clippy --all-targets -- -D warnings` and `cargo +nightly fmt` before you open a pull request, and describe what you changed and why.

By contributing, you agree to release your contribution into the public domain under the same terms as the rest of this repository.

## License

This project is released into the public domain under [the Unlicense](LICENSE). Copy it, change it, sell it, and do whatever you want with it. No attribution required.

Aster's other projects are licensed under AGPL v3. This one is not, so nothing here places any obligation on your own code.
