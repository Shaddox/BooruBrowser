# Booru Browser

A small terminal UI for searching and browsing Danbooru posts.

The app starts with a safe-rated Danbooru search, shows results in a list, and displays details for the selected post. From the terminal UI, you can page through results and open either the Danbooru post page or the best available image URL in your browser.

![Application Screenshot](/docs/screenshot-1.png)

## Requirements

- Rust 2024 edition toolchain
- Network access to `https://danbooru.donmai.us`
- A terminal that supports alternate-screen applications

## Run

```bash
cargo run
```

To check the project without running the UI:

```bash
cargo check
```

## Controls

| Key | Action |
| --- | --- |
| `/` | Focus the search box |
| `Enter` | Run search, or open the selected post from results |
| `j` / `Down` | Move selection down |
| `k` / `Up` | Move selection up |
| `n` / `Right` | Load next result page |
| `p` / `Left` | Load previous result page |
| `o` | Open selected Danbooru post |
| `i` | Open selected image URL |
| `r` | Reload current page |
| `?` | Toggle help |
| `q` | Quit |
| `Ctrl-C` | Quit |

Danbooru tag queries work as expected, for example:

```text
rating:safe landscape order:score
```

## Project Structure

```text
src/
  main.rs      startup and event loop
  api.rs       Danbooru constants and response model
  app.rs       application state and actions
  input.rs     keyboard handling
  terminal.rs  terminal setup and restore
  ui.rs        Ratatui drawing code
```

## Dependencies

- `tokio` for the async runtime
- `reqwest` for HTTP requests
- `serde` for JSON decoding
- `ratatui` for terminal UI rendering
- `crossterm` for terminal and keyboard handling
- `webbrowser` for opening post and image URLs
- `anyhow` for error handling
