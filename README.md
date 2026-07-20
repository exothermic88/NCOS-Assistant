# NCOS Assistant

A COSMIC panel applet that answers questions about NCOS. It chats with a
local (or LAN) [Ollama](https://ollama.com) model and grounds its answers in
your own documentation using retrieval-augmented generation: markdown files
are chunked by heading, embedded with `nomic-embed-text`, and the most
relevant chunks are given to the chat model with every question, cited as
`[source: file.md > heading]`.

## Requirements

- COSMIC desktop 1.3+ (for the frosted glass effect)
- Rust toolchain (`rustc`, `cargo`), plus [`just`](https://github.com/casey/just)
- Ollama

## Setup

```sh
# 1. Ollama service + models
sudo systemctl enable --now ollama
ollama pull nomic-embed-text   # embeddings (~270 MB)
ollama pull qwen3:4b           # chat model (~2.5 GB)

# 2. Build and install (installs to ~/.local)
just install
```

Then:

1. **COSMIC Settings → Desktop → Panel → Configure panel applets → Add
   "NCOS Assistant"** — the chat-bubble icon appears in the panel.
2. **COSMIC Settings → Appearance → Frosted Glass** — enable the effect for
   applets to get the frosted popup. (Without it the popup is normal opaque;
   that's the compositor setting, not the applet.)

## Feeding it your wiki

Drop `.md` (or `.txt`) files into `docs/` in this repo — that folder is
indexed by default. Add more folders in the applet's settings (gear icon).
The index rebuilds automatically when files change (checked when the popup
opens), or manually via **Settings → Rebuild**. The index is stored at
`~/.local/share/ncos-assistant/index.json`.

## Configuration

Settings live in the popup (gear icon) and persist via cosmic-config under
`~/.config/cosmic/io.github.exothermic88.ncos-assistant/v1/`:

- **Ollama server** — local by default (`localhost:11434`); toggle "Use
  remote server" to point at another machine on your network.
- **Models** — chat model (default `qwen3:4b`) and embedding model
  (default `nomic-embed-text`). Changing the embedding model invalidates
  the index and triggers a rebuild.
- **Context chunks** — how many document chunks are retrieved per question.

## Development

```sh
cargo check         # fast compile check
just build          # release build
just install        # install binary, desktop entry, icon
just uninstall
```

The applet is built on [libcosmic](https://github.com/pop-os/libcosmic)
(pinned by rev in `Cargo.toml`).
