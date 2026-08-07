# Changelog

All notable changes to this extension are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [0.2.0] - 2026-08-07

### Added
- Docs synced to `livetennisapi-mcp` 1.4.0 — 24 tools (was documented as 12):
  tournaments (`search_tournaments`, `get_tournament`), the 1968–2022 results
  archive (`search_archive_matches`, `get_archive_match`,
  `search_archive_players`, `get_archive_career`), cross-era head-to-head
  (`get_h2h`), ranking tables (`get_rankings`, PRO), per-player rankings as of
  a date (`get_player_rankings`, ULTRA), in-play statistics
  (`get_match_statistics`, ULTRA) and Match Charting Project shot data
  (`get_charting_player`, `get_charting_match`, ULTRA). The extension always
  runs the latest published server, so these tools are live without a code
  change.
- Quota table in the README (grid of 2026-08-06: FREE 100/day, BASIC 1,000/day,
  PRO 10,000/day, ULTRA 500,000/day) with free-key polling guidance.
- CI and license badges; docs/Discord/org links.
- `scripts/truthcheck.sh` ground-truth guard, wired into CI.

### Changed
- Node requirement raised to **v20+** (the server's engines floor as of
  `livetennisapi-mcp` 1.4.0).
- Tour phrasing: ATP, WTA, Challenger, ITF and juniors.
- Extension description reflects the wider surface (rankings, archive,
  statistics).

## [0.1.0] - 2026-08-02

### Added
- Initial release: Zed extension registering `livetennisapi-mcp` as a context
  server (stdio, key from Zed settings only), submitted to the Zed extensions
  registry.
