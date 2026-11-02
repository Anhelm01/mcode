# mcode

> Pure ASCII Cyberpunk Live Clock & Morphing Canvas for modern terminals.

Inspired by holographic code-masking effects (`ponytail.dev`), `mcode` renders dense systems code backgrounds illuminated by glowing TrueType vector silhouettes and organic 2D Perlin noise transitions (`Melt`).

Pure ASCII only — no unicode half-blocks (`▀`, `▄`), no Braille, no pseudo-graphics.

---

## Features

- **Live Cyberpunk Clock**: Runs out-of-the-box as an aesthetic desktop clock (`HH:MM` or `HH:MM:SS`).
- **Organic Melt Animation**: Shapes dissolve and emerge naturally through continuous 2D Perlin noise and burning edge contours.
- **Pure ASCII Vector Masks**: TrueType vector font rasterization (`fontdue`) dynamically scaled to any terminal geometry.
- **TrueColor Themes**: Built-in 24-bit palettes (`cyberpunk`, `holo`, `matrix`, `amber`, `monochrome`).
- **Double-Buffered Engine**: Flicker-free direct ANSI diff rendering with zero CPU waste at 60 FPS.
- **Custom Sequences & Images**: Display arbitrary text, cycling emoticons (`o_0`, `x_x`), or rasterize image silhouettes (PNG/JPG).

---

## Installation

```bash
# Clone and build
git clone https://github.com/Anhelm01/mcode.git
cd mcode
cargo build --release

# Copy or symlink binary into PATH
ln -s "$(pwd)/target/release/mcode" ~/.local/bin/mcode
```

---

## Usage

### 1. Live Clock (Default)
```bash
mcode
```

To display seconds (`HH:MM:SS`):
```bash
mcode --seconds
```

### 2. Custom Text & Words
```bash
mcode --text "98%"
mcode --text "o_0"
```

### 3. Morphing Sequence
```bash
mcode --sequence "o_0:6, 0_o:6, ._.:6, x_x:6, +_+:6"
```

### 4. Custom Palettes
```bash
mcode --palette holo
mcode --palette matrix
mcode --palette amber
```

---

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `--seconds` | Show seconds in clock mode (`HH:MM:SS`) | `false` |
| `-t, --text <TEXT>` | Display static text or phrase | — |
| `-s, --sequence <SEQ>` | Timed sequence of labels (e.g. `"A:5, B:5"`) | — |
| `-i, --image <PATH>` | Rasterize image silhouette mask (PNG/JPG) | — |
| `-p, --palette <NAME>` | Color theme: `cyberpunk`, `holo`, `matrix`, `amber`, `monochrome` | `cyberpunk` |
| `--transition <SEC>` | Smooth morph duration in seconds | `2.2` |
| `--fps <FPS>` | Target frame rate | `60` |
| `--speed <SPEED>` | Background vertical scroll speed | `0.0` (static) |
| `--edge-fade <FADE>` | Edge vignette fade factor | `0.08` |
| `--twinkle` | Enable background twinkling stars | `false` |

---

## Keybindings

- `q`, `Esc`, `Ctrl+C` — Clean exit with terminal screen restore.

---

## License

[MIT](LICENSE) © 2026 Anhelm
