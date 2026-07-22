# Live Tennis MCP Server — Zed Extension

Registers the [Live Tennis API](https://livetennisapi.com) MCP server as a context server in
[Zed](https://zed.dev), giving the agent panel real-time tennis data: live scores, fixtures,
players, odds and model win-probability.

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
| `check_api_status` | Verify your key and see remaining quota |

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
