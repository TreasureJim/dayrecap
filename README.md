# dayrecap

`dayrecap` is a notification client that sends you a notification when it is run. When the notification is clicked it runs a terminal command, that you specify, which will open an editor. Record your thoughts, events or anything else into the file and it will save into a log file. 

## Why

Why did I build this? For me I've been more successful at breaking bad habits when I am actively thinking about them so I can remind myself to not do them. This project specifically was made so I am more mindful or how I spend my time on my computer (so my day isn't wasted away by youtube etc) or when I am procrastinating something that I need to be doing I will have something reminding me of what I am actually doing.

---

# Installing

I am not currently distributing pre-built binaries. The easiest installation method currently is `cargo install`.

```sh
cargo install --git https://github.com/treasurejim/dayrecap`
```

---

### Options

| Option                                        | Description                                                                  |
| --------------------------------------------- | ---------------------------------------------------------------------------- |
| `-m, --messages-location <MESSAGES_LOCATION>` | File where recap logs are stored (default: `/home/judasmoses/RECAPLOG`) |
| `-e, --editor-command <EDITOR_COMMAND>`       | Command used to open the editor (default: `"alacritty -e nvim %p"`)          |
| `-h, --help`                                  | Print help information                                                       |
| `-V, --version`                               | Print version information                                                    |

The `%p` placeholder in the editor command is replaced with the path to the recap file.

---

## Automation

`dayrecap` is intended to be run on a schedule.

### Example `cron` entry (hourly)

```cron
0 * * * * dayrecap
```

### Example `cron` entry (daily)

```cron
0 21 * * * dayrecap
```

This way, you are periodically prompted to reflect without needing to think about it.
