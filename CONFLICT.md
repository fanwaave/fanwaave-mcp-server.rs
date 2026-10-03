# fanwaave/fanwaave-mcp-server.rs#2 — docs: add AGENTS.md and fleet sops env layout

head: chore/agents-md-and-sops-env  base: main  author: ORESoftware  updated: 2026-08-27T19:14:25Z
dir: /Users/maca5/codes/.claude-fleet/scratch/merge/fanwaave_fanwaave-mcp-server.rs__2

## conflicted files
- justfile

## base (main) last 8 commits
c4bdd6d DEN-965: harden Fanwaave MCP client and provider parity
e809600 Merge pull request #3 from fanwaave/agent/ores-sops-ensure-dec-20260828
28eb5a9 Cover remaining env/dec creation paths with ores-sops ensure-dec.
50fb547 Refuse unguarded env/dec mkdir before ores-sops.
944932f fix: recover from oversized MCP stdio frames
9b960c8 feat: specialize the hardened MCP server for fanwaave
b54877c feat: establish hardened organization MCP template

## head (chore/agents-md-and-sops-env) last 8 commits
fbcd3b4 Replace yanked chacha20 0.10.1 with 0.10.2.
0a934b6 Run just env-check in primary CI and ignore env/dec/.
02b46d2 docs: point AGENTS.md at the parent my-ai contract and codes symlink
c941aa0 docs: add functional programming coding patterns to AGENTS.md
ab5f299 docs: add AGENTS.md and fleet sops env layout
944932f fix: recover from oversized MCP stdio frames
9b960c8 feat: specialize the hardened MCP server for fanwaave
b54877c feat: establish hardened organization MCP template

## merge-base: 944932fc9fb916958a2b81905d7cba8d7c867e33

## PR diff stat (merge-base..head)
 .envrc                              |   7 +
 .github/workflows/ci.yml            |  10 +
 .github/workflows/secrets-audit.yml |  32 +++
 .gitignore                          |  18 ++
 .just/dotenv.py                     |  87 +++++++
 .just/env.just                      | 461 ++++++++++++++++++++++++++++++++++++
 .sops.yaml                          |  27 ++-
 AGENTS.md                           | 105 ++++++++
 Cargo.lock                          |   4 +-
 env/README.md                       |  28 +++
 env/enc/prod.env.enc                |  10 +
 flake.nix                           |  13 +-
 justfile                            |  76 +++---
 13 files changed, 840 insertions(+), 38 deletions(-)

## base diff stat (merge-base..base)
 .github/workflows/ci.yml |   13 +-
 Cargo.lock               | 1575 +++++++++++++++++++++++++++++++++++++++++++---
 Cargo.toml               |   11 +-
 README.md                |   74 ++-
 justfile                 |    4 +-
 mcp-fleet-profile.json   |  214 +++++++
 src/http.rs              |   10 +
 src/main.rs              |   21 +-
 src/spec.rs              |   61 ++
 tests/stdio_parity.rs    |  255 ++++++++
 10 files changed, 2132 insertions(+), 106 deletions(-)

## merge output
Auto-merging .github/workflows/ci.yml
Auto-merging Cargo.lock
Auto-merging justfile
CONFLICT (content): Merge conflict in justfile
Automatic merge failed; fix conflicts and then commit the result.
