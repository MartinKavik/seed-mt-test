# Seed multi-threading test

## Run

1. Open the project in your favorite IDE (I recommend [VS Code](https://code.visualstudio.com/) + [Rust Analyzer](https://rust-analyzer.github.io/)).
1. Open a new terminal tab / window and run: `cargo make serve`
1. Open a second terminal tab and run: `cargo make watch`

## Deploy

1. Run `cargo make verify` in your terminal to format and lint the code.
1. Run `cargo make build_release`.
1. Upload `index.html` and `pkg` into your server's public folder.
   - Don't forget to upload also configuration files for your hosting, see the [Netlify](https://www.netlify.com/) one below.

```toml
# netlify.toml
[[redirects]]
  from = "/*"
  to = "/index.html"
  status = 200
```

