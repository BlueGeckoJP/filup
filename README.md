# Scroll down for the README in English

# Filup: シンプルなファイル共有ソフト
FilupはLAN内で簡単にファイルを共有するために作ったアプリです
**ネットワーク上で公開して使用することを想定していない(仕様やセキュリティなどなど)のでネットワーク上で使用しないことを強くおすすめします**
このソフトウェアは趣味で作っているものなのでエラーがあったりコードが汚かったり、アプリの動作が不安定だったりします ご了承ください

## 依存関係
```toml
axum = { version = "0.7.5", features = ["multipart"] }
base64 = "0.22.1"
chrono = "0.4.38"
clap = { version = "4.5.16", features = ["derive"] }
futures-util = "0.3.30"
serde = { version = "1.0.209", features = ["derive"] }
tera = "1.20.0"
tokio = { version = "1.39.3", features = ["full"] }
tokio-stream = { version = "0.1.15", features = ["sync"] }
tower-http = { version = "0.5.2", features = ["fs", "trace"] }
tracing = "0.1.40"
tracing-subscriber = "0.3.18"
```

TypeScriptのファイルをJavaScriptにトランスパイルする際に使用するもの:
(最初からトランスパイルしたものをアップロードしているので**必須ではない**です)
```
pnpm, typescript
```

## Usage
* rustをインストールしてください cargoコマンドが使えたらオッケーです
* `
  cargo r --release
  `
  か
  `
  cargo b --release && ./target/release/filup
  `
  を実行してください

***

# Filup: Simple file sharing software
Filup is a file sharing software designed for easy file exchange within a LAN
**It is not intended to be used openly on a network (software specifications, security, etc.), so we recommend not using it!**
This software is **a hobby**, so there may be errors and the code may be difficult to read

## Requirements (Dependencies)
```toml
axum = { version = "0.7.5", features = ["multipart"] }
base64 = "0.22.1"
chrono = "0.4.38"
clap = { version = "4.5.16", features = ["derive"] }
futures-util = "0.3.30"
serde = { version = "1.0.209", features = ["derive"] }
tera = "1.20.0"
tokio = { version = "1.39.3", features = ["full"] }
tokio-stream = { version = "0.1.15", features = ["sync"] }
tower-http = { version = "0.5.2", features = ["fs", "trace"] }
tracing = "0.1.40"
tracing-subscriber = "0.3.18"
```

For TypeScript builds:
```
pnpm, typescript
```

## Usage
* Please install rust(cargo)
* `
  cargo r --release
  `
  or
  `
  cargo b --release && ./target/release/filup
  `
