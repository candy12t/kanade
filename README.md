# kanade

Switch the macOS input source to **英数 / かな** with the Command keys.

- **Tap left Command** → 英数 (Eisu)
- **Tap right Command** → かな (Kana)

## Requirements

- macOS on Apple Silicon
- Accessibility permission

## Install

```sh
curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/candy12t/kanade/releases/latest/download/kanade-installer.sh | sh
```

Installs to `~/.local/bin` (make sure it's on your `PATH`). Or build from source with `cargo install --path .`.

## Usage

Install the background agent (starts now and at every login):

```sh
kanade install
```

The first run prompts for Accessibility permission. Grant it under **System Settings → Privacy & Security → Accessibility**, then run `kanade restart`. The binary is unsigned, so you'll need to re-grant after each update.

| Command            | Description                               |
| ------------------ | ----------------------------------------- |
| `kanade run`       | Run in the foreground (`Ctrl-C` to quit). |
| `kanade install`   | Install and start the launchd agent.      |
| `kanade uninstall` | Stop and remove the agent.                |
| `kanade restart`   | Restart the agent.                        |
| `kanade status`    | Show the agent status.                    |

## License

[MIT](LICENSE) © Takumi Kanada
