app-id := "io.github.exothermic88.ncos-assistant"
bin := "ncos-assistant"

build:
    cargo build --release

install: build
    install -Dm755 target/release/{{bin}} ~/.local/bin/{{bin}}
    install -Dm644 data/{{app-id}}.desktop ~/.local/share/applications/{{app-id}}.desktop
    install -Dm644 data/icons/{{app-id}}-symbolic.svg ~/.local/share/icons/hicolor/scalable/apps/{{app-id}}-symbolic.svg

uninstall:
    rm -f ~/.local/bin/{{bin}}
    rm -f ~/.local/share/applications/{{app-id}}.desktop
    rm -f ~/.local/share/icons/hicolor/scalable/apps/{{app-id}}-symbolic.svg

run:
    cargo run --release
