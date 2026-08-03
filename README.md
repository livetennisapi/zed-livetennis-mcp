# Live Tennis MCP Server — Zed Extension

Registers the [Live Tennis API](https://livetennisapi.com) MCP server as a context server in
[Zed](https://zed.dev), giving the agent panel real-time tennis data: live scores, players and
fixtures on the free plan. Match timelines, market prices and model analysis require a paid plan.

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

Requires **Node.js v18+**. The [`livetennisapi-mcp`](https://www.npmjs.com/package/livetennisapi-mcp)
npm package is installed and kept up to date automatically by Zed — you do not need to install it.

## Settings

| Setting | Type | Required | Description |
|---------|------|----------|-------------|
| `api_key` | string | yes | Your Live Tennis API key (`twjp_…`) |

## Tools

All 12 tools are **read-only**.

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

Tools above your plan are not errors: they return a plain-English note explaining which plan is
needed. Run `check_api_status` to see which plan your key is on.

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

## License

MIT — see [LICENSE](LICENSE).

## Affiliate program

Know developers who need tennis data? The [affiliate program](https://affiliates.livetennisapi.com/program) pays 51% recurring commission for the life of every referred subscription — 30-day cookie, and the people you refer get 10% off.
