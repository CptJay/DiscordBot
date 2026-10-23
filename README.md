# Discord Bot in Rust

> **Status: discontinued.** Not actively maintained.

A simple Discord bot written in Rust, built to learn Rust and the Discord API with [`serenity`](https://crates.io/crates/serenity).
It looks up League of Legends players through the [Riot Games API](https://developer.riotgames.com/apis).

## Commands

| Command | Does |
| --- | --- |
| `&ping` | Replies `Pong!` |
| `&echo <text>` | Repeats the text |
| `&lookup <name>#<tag>` | Summoner level, profile icon and PUUID (EUW) |

Riot ID to PUUID lookups are cached in `cache/user_cache.json`.

## Running

Create a `.env` file in the project root:

```
DISC_TOKEN=your-discord-bot-token
RIOT_TOKEN=your-riot-api-key
```

Then:

```
cargo run
```

## Known limits

- The region is fixed to EUW.
- Most Riot endpoint groups are stubs; only the summoner lookup is wired to a command.
- The bot requests all gateway intents, so enable the privileged intents (including Message Content) in the Discord developer portal.
