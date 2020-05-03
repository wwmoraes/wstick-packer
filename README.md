<h3 align="center">wstick-packer</h3>

<div align="center">

[![Status](https://img.shields.io/badge/status-active-success.svg)]()
[![GitHub Issues](https://img.shields.io/github/issues/wwmoraes/wstick-packer.svg)](https://github.com/wwmoraes/wstick-packer/issues)
[![GitHub Pull Requests](https://img.shields.io/github/issues-pr/wwmoraes/wstick-packer.svg)](https://github.com/wwmoraes/wstick-packer/pulls)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](/LICENSE)

</div>

---

<p align="center"> generates a whatsapp sticker pack in wstick-compatible format
    <br>
</p>

## 📝 Table of Contents

- [About](#about)
- [Getting Started](#getting_started)
- [Deployment](#deployment)
- [Usage](#usage)
- [Built Using](#built_using)
- [TODO](../TODO.md)
- [Contributing](../CONTRIBUTING.md)

## 🧐 About <a name = "about"></a>

Tool that generates a wstick-compatible json file with images and pack info ready to be shared
and installed using WSticK app on mobile phones.

## 🏁 Getting Started <a name = "getting_started"></a>

clone the repository and then `cargo run <pack-path1> [pack-pack2..pack-pathN]`

### Prerequisites

To install rust toolchain do

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

you may also use your OS-specific tool to do so (e.g. apt, pacman, brew).

### Installing

To install clone the repository and

```shell
cargo install --path .
```

then

```
wstick-packer
```

Should work everywhere :D

## 🔧 Running the tests <a name = "tests"></a>

Run `cargo test` :D

## 🎈 Usage <a name="usage"></a>

A typical package folder contains a `pack.json` file and any number of jpeg files (ended as `.jpg`).
The tool will then read the info from the json, convert the images to base64 and create the
`generated.json` with all images encoded as base64.

To prevent duplicates, the code hashes the image contents and only adds it once.

### Pack file

The `pack.json` file has these fields:

```json
{
  "name": "Your Pack Name",
  "identifier": "a-unique-identifier-for-your-pack",
  "publisher": "your-name-or-any-other-form-of-identification",
  "ios_app_store_link": "link-to-ios-app",
  "android_play_store_link": "link-to-android-app"
}
```

Both `ios_app_store_link` and `android_play_store_link` default to wstick, so you can omit those.

The resulting `generated.json` will have the `pack.json` + default fields + those extras:

```json
{
  "tray_image": "base64-of-an-image",
  "stickers": [
    {
      "image_data": "base64-of-an-image",
      "emojis": []
    }
  ]
}
```

### Tray image

Sticker packs need a tray image, which will be shown on WhatsApp tray to identify your pack.
the `tray_image` field will be filled with the base64-encoded contents of `<psck-dir>/tray_image.jpg`.

### Default pack info

The default values for a pack are:

```json
{
  "name": "unnamed",
  "identifier": "unidentified",
  "publisher": "unknown",
  "tray_image": "",
  "stickers": [],
  "ios_app_store_link": "https://itunes.apple.com/app/wstick/id1442273161?mt=8",
  "android_play_store_link": "https://play.google.com/store/apps/details?id=com.wstick.hk"
}
```

Extra fields will be totally ignored 🖤

## ⛏️ Built Using <a name = "built_using"></a>

- [Rust](https://www.rust-lang.org/) - base language
- [Serde](https://serde.rs/) - JSON (de)serializer library
