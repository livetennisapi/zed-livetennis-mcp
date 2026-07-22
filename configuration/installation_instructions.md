# Live Tennis MCP Server — Setup

Gives Zed's agent real-time tennis data: live scores, players and fixtures on the free plan.
Match timelines, market prices and model analysis require a paid plan.

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

| Tool | Description | Plan |
|------|-------------|------|
| `get_live_matches` | Matches currently in play | Free |
| `get_upcoming_matches` | Matches scheduled to start soon | Free |
| `get_match` | Full detail for one match | Free |
| `get_match_score` | Current score for one match | Free |
| `search_players` | Find players by name | Free |
| `get_player` | Profile and ranking for one player | Free |
| `get_fixtures` | Scheduled fixtures | Free |
| `check_api_status` | Check reachability and which plan your key is on | Free |
| `get_recent_results` | Recently completed matches | Basic |
| `get_match_events` | Match timeline: breaks, games won, momentum runs | Pro |
| `get_match_odds` | Match-winner market prices (bid/ask/mid) | Pro |
| `get_match_analysis` | Model analysis and win probability | Ultra |

All tools are **read-only**. Tools above your plan are not errors — they return a plain-English
note explaining which plan is needed. Run `check_api_status` to see your current plan.

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
