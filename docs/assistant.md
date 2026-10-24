# NCOS Assistant

The NCOS Assistant is the built-in AI assistant in the NCOS panel. It answers
questions about NCOS using the NCOS documentation and also works as a general chat
assistant. It runs fully offline.

## What the NCOS Assistant Does

- **NCOS questions:** The NCOS Assistant answers questions about NCOS shortcuts, how-tos, and system tasks using the built-in NCOS documentation.
- **General chat:** The NCOS Assistant also answers general questions, like command syntax or explanations.
- **Privacy:** The NCOS Assistant runs entirely on your computer using Ollama. Nothing you type is sent to the internet, and no account or API key is needed.
- **Example:** If you ask "How do I switch to the red theme?", the Assistant tells you to press `Super+Shift+]` to cycle themes.

## Set Up the NCOS Assistant and Ollama

Ollama ships on the NCOS ISO since NCOS 2.1.3. Before first use, start the Ollama service and download the two models the Assistant uses.

Start Ollama now and at every boot:

```bash
sudo systemctl enable --now ollama
```

Download the embedding model and the chat model:

```bash
ollama pull embeddinggemma:300m
ollama pull llama3.2:1b
```

## Add the NCOS Assistant to the Panel

To add the NCOS Assistant to the panel, open COSMIC Settings, go to Desktop, then Panel, then Configure panel applets, and add "NCOS Assistant". The Assistant icon then appears in the panel. Click the icon to open the chat.

## NCOS Assistant Settings

Open the NCOS Assistant settings with the gear icon in the chat popup. The settings are saved in `~/.config/cosmic/io.github.exothermic88.ncos-assistant/v1/`.

- **Ollama server:** The Assistant uses the local Ollama at `localhost:11434` by default. Turn on "Use remote server" to use Ollama on another computer on your network.
- **Chat model:** The default chat model is `llama3.2:1b`.
- **Embedding model:** The default embedding model is `embeddinggemma:300m`. Changing it rebuilds the document index.
- **Context chunks:** Sets how many pieces of documentation are given to the model for each question.
- **Show sources:** Shows which documentation pages an answer came from.

## Add Your Own Documents to the NCOS Assistant

The NCOS Assistant reads the NCOS documentation from `/usr/share/ncos-assistant/docs`. To add your own notes, put `.md` or `.txt` files in a folder and add that folder in the Assistant settings (gear icon). The index rebuilds automatically when files change, checked each time the popup opens. You can also rebuild it by hand with the Rebuild button in settings. The index is stored at `~/.local/share/ncos-assistant/index.json`.

## NCOS Assistant Troubleshooting and Tips

- **Can't reach Ollama:** If the Assistant says it can't reach Ollama, start the service with `sudo systemctl enable --now ollama`.
- **Slow first answer:** The first answer after boot can take a few seconds while the model loads. After that it stays fast.
- **Best results:** The Assistant uses a small model, so ask short, focused questions.
- **Deep troubleshooting:** For complex problems, the Arch Wiki at https://wiki.archlinux.org/ is the best resource.
