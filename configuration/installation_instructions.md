# Live Tennis MCP Server — Setup

Gives Zed's agent real-time and historical tennis data: live scores, players, fixtures and
tournaments on the free plan. Recent results, the 1968–2022 archive and head-to-heads need BASIC;
match timelines, market prices and rankings need PRO; in-play statistics, charting and model
analysis need ULTRA.

## 1. Get an API key

Sign up for a **free** key at:

**<https://livetennisapi.com/subscribe/free>**

Your key looks like `twjp_…`.

## 2. Add the key to your Zed settings

Open `zed: open settings` from the command palette and add:

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

That's the only required setting.

## Requirements

- **Node.js** v20 or newer.

The `livetennisapi-mcp` npm package is downloaded and kept up to date automatically — you do not
need to install it yourself.

## Available tools

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
| `get_archive_match` | One archive result, with era serve stats | Basic |
| `search_archive_players` | Archive player bios — hand, DOB, career-high rank | Basic |
| `get_archive_career` | Career W-L, titles and serve aggregates | Basic |
| `get_h2h` | Cross-era head-to-head — archive + current | Basic |
| `get_match_events` | Match timeline: breaks, games won, momentum runs | Pro |
| `get_match_odds` | Match-winner market prices (bid/ask/mid) | Pro |
| `get_rankings` | Full published ranking table per system, any week | Pro |
| `get_player_rankings` | Per-player ranking records as of any date | Ultra |
| `get_match_statistics` | In-play statistics — aces, serve split, break points | Ultra |
| `get_charting_player` | Career shot-level profile (Match Charting Project) | Ultra |
| `get_charting_match` | One charted match, every stat family | Ultra |
| `get_match_analysis` | Model analysis and win probability | Ultra |

The six Basic history tools are also unlocked by any History plan, which works on top of a free
key. Tools above your plan are not errors — they return a plain-English note explaining which plan
is needed. Run `check_api_status` to see your current plan.

## Troubleshooting

**"Missing `settings`" or the setup panel keeps appearing** — the `api_key` field is absent or
empty. Re-check the JSON above; the key belongs under `settings`, not at the top level of the
context server entry.

**Authentication errors from the tools** — run `check_api_status` in the agent panel; it reports
whether the key is valid and how much quota remains.

**Server fails to start** — confirm `node --version` is 20 or newer.

## Links

- Live Tennis API docs: <https://docs.livetennisapi.com>
- npm package: <https://www.npmjs.com/package/livetennisapi-mcp>
