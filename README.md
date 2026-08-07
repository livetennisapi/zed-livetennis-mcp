# Live Tennis MCP Server — Zed Extension

[![ci](https://github.com/livetennisapi/zed-livetennis-mcp/actions/workflows/ci.yml/badge.svg)](https://github.com/livetennisapi/zed-livetennis-mcp/actions/workflows/ci.yml)
[![license: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Registers the [Live Tennis API](https://livetennisapi.com) MCP server as a context server in
[Zed](https://zed.dev), giving the agent panel real-time and historical tennis for ATP, WTA,
Challenger, ITF and juniors: live scores, players, fixtures and tournaments on the free plan;
recent results, a 1968–2022 archive and head-to-heads on BASIC; odds, events and rankings on PRO;
in-play statistics, charting and model analysis on ULTRA — 24 read-only tools.

## Installation

1. Install this extension from Zed's extension registry (`zed: extensions`).
2. Get a **free** API key at <https://livetennisapi.com/subscribe/free>.
3. Add it to your Zed settings:

```json
{
  "context_servers": {
    "livetennis-mcp": {
      "settings": {
        "api_key": "twjp_your_key_here"
      }
    }
  }
}
```

Requires **Node.js v20+**. The [`livetennisapi-mcp`](https://www.npmjs.com/package/livetennisapi-mcp)
npm package is installed and kept up to date automatically by Zed — you do not need to install it.

## Settings

| Setting | Type | Required | Description |
|---------|------|----------|-------------|
| `api_key` | string | yes | Your Live Tennis API key (`twjp_…`) |

## Tools

All 24 tools are **read-only**.

| Tool | Description | Plan |
|------|-------------|------|
| `get_live_matches` | Matches currently in play | Free |
| `get_upcoming_matches` | Matches scheduled to start soon | Free |
| `get_match` | Full detail for one match | Free |
| `get_match_score` | Current score for one match — fastest read | Free |
| `search_players` | Find players by name | Free |
| `get_player` | Profile, ranking, country, handedness | Free |
| `get_fixtures` | Scheduled fixtures | Free |
| `search_tournaments` | Tournament catalogue — surface, location, category | Free |
| `get_tournament` | One tournament by its stable id | Free |
| `check_api_status` | Check reachability and which plan your key is on | Free |
| `get_recent_results` | Recently completed matches | Basic |
| `search_archive_matches` | Results archive (1968–2022) with ranks and seeds at the time | Basic |
| `get_archive_match` | One archive result, with serve stats where the era recorded them | Basic |
| `search_archive_players` | Archive player bios — hand, DOB, career-high rank | Basic |
| `get_archive_career` | Career W-L, titles and serve aggregates over the archive | Basic |
| `get_h2h` | Cross-era head-to-head — archive + current, one record | Basic |
| `get_match_events` | Match timeline: breaks, games won, momentum runs | Pro |
| `get_match_odds` | Match-winner market prices (bid/ask/mid) | Pro |
| `get_rankings` | Full published ranking table per system (ATP, WTA, ITF circuits), any week | Pro |
| `get_player_rankings` | Point-in-time ranking records for specific players, as of any date | Ultra |
| `get_match_statistics` | In-play statistics — aces, serve split, hold/break %, break points | Ultra |
| `get_charting_player` | Career shot-level profile from the Match Charting Project | Ultra |
| `get_charting_match` | One charted match, every stat family, per-set split | Ultra |
| `get_match_analysis` | Model analysis and win probability | Ultra |

The six Basic history tools are also unlocked by any History plan, which works on top of a free
key. Tools above your plan are not errors: they return a plain-English note explaining which plan
is needed. Run `check_api_status` to see which plan your key is on.

## Quotas

| Plan | Per minute | Per day | Price |
|------|-----------:|--------:|-------|
| FREE | 30 | 100 | $0 |
| BASIC | 60 | 1,000 | $9.99/mo |
| PRO | 300 | 10,000 | $29.99/mo |
| ULTRA | 600 | 500,000 | $99.99/mo |

On a free key (100 requests/day), poll no faster than every 15 minutes; for an always-on
dashboard, BASIC is recommended.

## How it works

Zed's extension API starts a context server by spawning a **process**, so the server runs locally
over stdio rather than as a remote HTTP endpoint. On startup the extension:

1. resolves the latest published version of `livetennisapi-mcp` and installs or updates it via
   Zed's npm APIs (the server is never vendored into this extension);
2. spawns it with the Node binary Zed provides;
3. passes your `api_key` to that process as `LIVETENNISAPI_KEY`.

Your key is read **only** from your Zed settings and is never read from the ambient environment.

## Development

```bash
rustup target add wasm32-wasip2
cargo build --release --target wasm32-wasip2
```

To test locally: `zed: extensions` → **Install Dev Extension** → select this directory.

## Links

- Docs: <https://docs.livetennisapi.com>
- Free key: <https://livetennisapi.com/subscribe/free>
- Discord: <https://discord.gg/f8WUZHgDm6>
- GitHub org: <https://github.com/livetennisapi>
- Server source: <https://github.com/livetennisapi/livetennisapi-mcp>

## License

MIT — see [LICENSE](LICENSE).

## Affiliate program

Know developers who need tennis data? The [affiliate program](https://affiliates.livetennisapi.com/program) pays 51% recurring commission for the life of every referred subscription — 30-day cookie, and the people you refer get 10% off.
