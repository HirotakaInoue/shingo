# shingo

Waveshare RP2040-Zeroを使用した、ボタン入力とLED制御の組み込みRustプロジェクトです。

## GPIO

| 用途 | GPIO | 設定 |
| --- | ---: | --- |
| 赤ボタン | GP10 | プルアップ入力（Lowで押下） |
| 緑ボタン | GP11 | プルアップ入力（Lowで押下） |
| 黄ボタン | GP12 | プルアップ入力（Lowで押下） |
| 赤LED | GP13 | プッシュプル出力（Highで点灯） |
| 緑LED | GP14 | プッシュプル出力（Highで点灯） |
| 黄LED | GP15 | プッシュプル出力（Highで点灯） |

ボタンはGPIOとGNDの間に接続します。外付けLEDには適切な電流制限抵抗を使用してください。

## ビルド

RustとRP2040向けターゲットを準備します。

```sh
rustup target add thumbv6m-none-eabi
cargo build --release --locked
```

生成されるELFファイルは `target/thumbv6m-none-eabi/release/shingo` です。

`.cargo/config.toml` のrunnerを使ってボードへ書き込む場合は、`elf2uf2-rs` をインストールし、RP2040-ZeroをBOOTSELモードで接続して実行します。

```sh
cargo install elf2uf2-rs
cargo run --release --locked
```

## メモリ

リンカ用のメモリ配置は `memory.x` で定義しています。

| 領域 | 開始アドレス | サイズ | 用途 |
| --- | --- | ---: | --- |
| BOOT2 | `0x10000000` | 256 B | RP2040の第2段ブートローダー |
| FLASH | `0x10000100` | 2 MiB - 256 B | プログラムと読み取り専用データ |
| RAM | `0x20000000` | 264 KiB | スタック、静的データ、実行時データ |

ビルドターゲットは `.cargo/config.toml` で `thumbv6m-none-eabi` に固定され、`link.x` を介してこのメモリ配置が適用されます。dev/releaseともにpanic時はabortします。
