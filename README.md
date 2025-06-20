# RLHighlight

[![License](https://img.shields.io/badge/License-MIT-green.svg)](#)
![Platform](https://img.shields.io/badge/platform-Windows-lightgray.svg)
[![Rust](https://img.shields.io/badge/Rust-%23000000.svg?e&logo=rust&logoColor=white)](#)
[![Tauri](https://img.shields.io/badge/Tauri-%2324C8D8.svg?logo=tauri&logoColor=white)]()
[![Twitter badge][]][Twitter link]

**Rocket League大会用Highlight自動clip・再生システム**

BakkesModプラグインからUDPデータを受信し、OBSと連携してリプレイバッファの録画・再生を自動化するTauriデスクトップアプリケーションです
![image](https://github.com/user-attachments/assets/901dda00-ff01-492a-b67e-8cf1c9684dfe)

## ✨ 主要機能

- 🎯 **自動ハイライト録画**: Goal・Epic Saveを自動検知して録画
- ⏰ **設定可能な録画遅延**: 1-30秒の範囲で録画タイミングを調整
- 📹 **リアルタイム動画管理**: 録画済みハイライトのリスト表示・再生
- 🔄 **OBS Studio完全連携**: WebSocket経由でのシームレス制御
- 🚀 **自動更新機能**: GitHub Releases連携で最新版を自動取得
- 🎨 **直感的なUI**: Dark Theme対応の使いやすいインターフェース

## 📋 前提条件

### 必須ソフトウェア
- **OBS** (WebSocketサーバー有効化)
- **[Mugi](https://github.com/Kazuryu0907/Mugi)** (BakkesMod plugin)
- **[VLC meida player](https://www.videolan.org/vlc/index.ja.html)** (Highlight再生用)

### OBS Studio設定
OBSを起動
# Websocket
1. `ツール` → `WebSocketサーバー設定` を開く
2. `WebSocketサーバーを有効にする` をチェック
3. ポート番号を `4455` に設定（変更可能）
4. パスワードを設定（任意）
# Replay buffer
1. `設定` → `出力`タブを開く
2. `リプレイバッファ` → `リプレイバッファを有効にする`
3. 最大リプレイ時間を自分で設定 (おすすめは`5 s`)

## 🚀 インストール

### 1. リリースからダウンロード
[Releases](https://github.com/yourusername/rl_replay/releases)から最新版をダウンロード

### 2. インストール実行
ダウンロードした `.msi` ファイルを実行してインストール

## 🎮 使用方法

### 1. 起動
1. **OBS** が起動していることを確認
2. **RLHighlight** を起動
3. OBS接続画面でポート番号（デフォルト: 4455）とパスワード（任意）を入力
4. 「OBSに接続」をクリック
5. OBSにVLCソース`RL_REPLAY_VLC_SOURCE`が自動で生成される

### 2. Highlight録画
1. Rocket Leagueを起動してゲームを開始
2. GoalやEpic Saveが発生すると自動的にclip
3. ダッシュボードの「録画済み動画」に表示される

### 3. ハイライト再生
1. 「ハイライト再生」ボタンをクリック
2. OBSのVLCソースで自動再生

### 4. 設定変更
- **録画遅延時間**: ダッシュボードの「設定」で1-30秒の範囲で調整
- イベント検知からclipまでの遅延時間を設定可能

## ⚙️ 設定

### UDP設定
- **ポート**: 12345 (固定)
- **プロトコル**: UDP
- Mugiがこのポートにイベントデータを送信

## 🛠️ 開発

### 技術スタック
- **フロントエンド**: React 19 + TypeScript + Vite + Tailwind CSS v4
- **バックエンド**: Rust 2024 Edition + Tauri v2
- **ビルドシステム**: Bun + Tauri v2

### 開発環境セットアップ

#### 前提条件
- Node.js & Bun
- Rust (latest stable)
- Visual Studio Build Tools (Windows)

#### クローン & セットアップ
```bash
git clone https://github.com/Kazuryu0907/RLHighlight.git
cd RLHighlight
bun i
```

#### 開発サーバー起動
```bash
bun run tauri dev
```

#### ビルド
```bash
bun run tauri build
```

#### Lint
```bash
# Rustコード
cd src-tauri && cargo clippy --fix
```

### プロジェクト構成
```
RLHighlight/
├── src/                    # React フロントエンド
│   ├── components/         # UIコンポーネント
│   │   ├── LoginForm.tsx   # OBSログインフォーム
│   │   └── Dashboard.tsx   # メインダッシュボード
│   ├── App.tsx            # メインアプリケーション
│   └── main.tsx           # エントリーポイント
├── src-tauri/             # Rust バックエンド
│   └── src/
│       ├── lib.rs         # メイン・状態管理
│       ├── obs.rs         # OBS WebSocket制御
│       ├── udp.rs         # UDP受信サーバー
│       ├── vlc_manager.rs # VLC動画管理
│       └── mugi_schema.rs # コマンドスキーマ
└── public/                # 静的ファイル
```

## 🐛 トラブルシューティング

### 接続エラー
- OBS StudioのWebSocketサーバーが有効になっているか確認
- ポート番号が正しいか確認（デフォルト: 4455）
- Replay bufferが有効になっているか確認

### Highlight再生されない
- OBS Studioで `RL_REPLAY_VLC_SOURCE` という名前のVLCソースが作成されているか確認
- VLCソースがシーンに追加されているか確認
- VLCソースのレイヤーが一番上に来ているか確認

## 📄 ライセンス

このプロジェクトは[MIT License](LICENSE)の下で公開されています。

## 🤝 Contribute

1. このリポジトリをフォーク
2. 機能ブランチを作成 (`git checkout -b feature/AmazingFeature`)
3. 変更をコミット (`git commit -m 'Add some AmazingFeature'`)
4. ブランチにプッシュ (`git push origin feature/AmazingFeature`)
5. プルリクエストを作成

## 📞 サポート

問題や質問がある場合は、[Issues](https://github.com/Kazuryu0907/RLHighlight/issues)で報告してください。

[Twitter badge]: https://img.shields.io/twitter/url?label=kazuryu_rl&style=social&url=https%3A%2F%2Ftwitter.com%2Fkazuryu_rl
[Twitter link]: https://twitter.com/intent/follow?screen_name=kazuryu_rl
