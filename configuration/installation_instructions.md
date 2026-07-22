# Live Tennis MCP Server — Setup

Gives Zed's agent real-time tennis data: live scores, upcoming and recent matches, players,
odds and model win-probability.

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

- **Node.js** v18 or newer.

The `livetennisapi-mcp` npm package is downloaded and kept up to date automatically — you do not
need to install it yourself.

## Available tools

| Tool | Description |
|------|-------------|
| `get_live_matches` | Matches currently in play |
| `get_upcoming_matches` | Matches scheduled to start soon |
| `get_match` | Full detail for one match |
| `get_match_score` | Current score for one match |
| `search_players` | Find players by name |
| `get_player` | Profile and ranking for one player |
| `get_fixtures` | Scheduled fixtures |
| `get_recent_results` | Recently completed matches |
| `get_match_events` | Point-by-point / event feed for a match |
| `get_match_odds` | Bookmaker odds for a match |
| `get_match_analysis` | Model win-probability and analysis |
| `check_api_status` | Verify your key and see quota |

All tools are **read-only**.

## Troubleshooting

**"Missing `settings`" or the setup panel keeps appearing** — the `api_key` field is absent or
empty. Re-check the JSON above; the key belongs under `settings`, not at the top level of the
context server entry.

**Authentication errors from the tools** — run `check_api_status` in the agent panel; it reports
whether the key is valid and how much quota remains.

**Server fails to start** — confirm `node --version` is 18 or newer.

## Links

- Live Tennis API: <https://livetennisapi.com>
- npm package: <https://www.npmjs.com/package/livetennisapi-mcp>
