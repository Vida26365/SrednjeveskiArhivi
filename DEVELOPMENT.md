# Development

## Preparation

Please ensure you have the following installed:

* [Rust & Cargo](https://www.rust-lang.org/tools/install)
* [Node.js & npm](https://nodejs.org/en/download/)
* [Dioxus CLI](https://dioxuslabs.com/learn/0.6/getting_started/#install-the-dioxus-cli)
* [WebView](https://dioxuslabs.com/learn/0.6/getting_started/#platform-specific-dependencies)

Once the tools are installed, run the following command to install npm dependencies:

```bash
npm install
```

## Development

Run the following command to start the Tailwind CSS compiler:

```bash
npm run watch
```

In another terminal, run the following command to start the app:

```bash
dx serve
```

To run for a different platform, use the `--platform platform` flag. E.g.

```bash
dx serve --platform desktop
```

When using JetBrains IDEs or VS Code, you can use pre-configured tasks to run the above commands.

## Building

Run the following command to build the Tailwind CSS:

```bash
npm run build
```

Run the following command to build and package the app:

```bash
dx bundle
```

## Formatting

Run the following command to format the code:

```bash
cargo +nightly fmt
```

Run the following command to statically analyze the code:

```bash
cargo clippy
```
