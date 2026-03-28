# cTUI WASM Browser Example

This example demonstrates running a cTUI terminal application in the browser using WebAssembly and HTML5 Canvas.

## Prerequisites

- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) installed
- A modern web browser

```bash
cargo install wasm-pack
```

## Building

From the repository root, build the WASM module:

```bash
cd ctui-wasm
wasm-pack build --target web --out-dir examples/browser/pkg
```

Or from the workspace root:

```bash
wasm-pack build ctui-wasm --target web --out-dir ctui-wasm/examples/browser/pkg
```

## Running

You need a local web server to serve the files (browsers require HTTPS or localhost for WASM).

### Option 1: Using npx serve

```bash
cd ctui-wasm/examples/browser
npx serve .
```

Then open http://localhost:3000 in your browser.

### Option 2: Using Python

```bash
cd ctui-wasm/examples/browser
python3 -m http.server 8080
```

Then open http://localhost:8080 in your browser.

### Option 3: Using cargo install miniserve

```bash
cargo install miniserve
cd ctui-wasm/examples/browser
miniserve .
```

## Features

- Terminal-like canvas rendering
- Keyboard input handling (arrow keys, typing, enter, backspace)
- Mouse click to move cursor
- Cursor blinking animation
- FPS counter
- Responsive resize handling

## Key Bindings

| Key | Action |
|-----|--------|
| Arrow keys | Move cursor |
| Type | Write characters |
| Enter | New line |
| Backspace | Delete character |
| Q / Esc | Clear screen |

## Architecture

The example uses:
- `ctui-wasm` - WASM bindings for cTUI
- `CanvasBackend` - HTML5 Canvas rendering backend
- `index.js` - JavaScript glue code for WASM initialization and event handling

## Customization

Edit `index.js` to:
- Change color scheme (`CONFIG` object)
- Modify font settings
- Add custom key handlers
- Implement different rendering logic
