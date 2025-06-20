# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## プロジェクト概要

Rocket League（RL）大会用のハイライト自動作成・再生システムのTauriデスクトップアプリケーション。BakkesModプラグインからのUDPデータを受信し、OBS Studioと連携してリプレイバッファの録画・再生を自動化する。

## アーキテクチャ

### フロントエンド・バックエンド構成
- **フロントエンド**: React + TypeScript + Vite（`src/`）
- **バックエンド**: Rust + Tauri（`src-tauri/src/`）
- **ビルドシステム**: Tauri v2 + Bun

### 非同期イベント処理アーキテクチャ
Tauriランタイム内で非同期タスクが並行実行：

1. **UDP受信** (`udp.rs`) - ポート12345でBakkesModイベント受信
2. **OBS制御** (`obs.rs`) - WebSocket経由でOBS Studio制御（localhost:4455）
3. **VLC管理** (`vlc_manager.rs`) - リプレイファイルパスのキューイング管理
4. **イベントスキーマ** (`mugi_schema.rs`) - BakkesModからのJSONコマンド解析

### イベントフロー
1. BakkesModがUDPでゲームイベント送信（Goal/Epic Save等）
2. 3秒遅延後にOBSリプレイバッファ保存トリガー
3. OBSイベントストリームでファイルパス取得
4. VlcManagerがパスを静的キューに蓄積
5. デバッグコマンド実行でOBS VLCソース経由で全動画再生

## 開発コマンド

### 開発環境起動
```bash
bun tauri dev
```

### ビルド
```bash
# フロントエンド＋バックエンドビルド
bun run build
bun tauri build

# Rustバックエンドのみ
cd src-tauri && cargo build
```

### テスト
```bash
# Rustテスト
cd src-tauri && cargo test

# 個別テスト実行
cd src-tauri && cargo test test_all
```

## 重要な実装詳細

### Tauri統合
- `lib.rs`のrunメソッドでTauriアプリ初期化前に非同期システム起動
- `console_subscriber::init()`でtokioトレーシング有効化
- Rustコードは`tauri::async_runtime::block_on`内で実行

### 外部依存関係
- **OBS Studio**: WebSocketサーバー必須（ポート4455）
- **カスタムOBWS**: `fix-VlcSource`ブランチ使用
- **BakkesMod**: Rocket Leagueプラグイン
- **sccache**: Rustコンパイル高速化（Windows環境）

### 静的状態管理
- VLCファイルパスは`LazyLock<Mutex<Vec<PathBuf>>>`でグローバル管理
- OBSイベントリスナーは専用クライアント接続で非ブロッキング実装

### デバッグ・テストツール
- `udp_cli.py`: 手動UDPコマンド送信（継続利用）
- デバッグコマンド: `{"cmd":"dbg"}` - 蓄積動画の再生実行
- テストコマンド: `{"cmd":"scored"}`, `{"cmd":"epicSave"}`

## 設定要件

### 必須サービス
- OBS Studio（WebSocketサーバー有効化）
- BakkesMod（Rocket League起動時）

### Bunコマンド使用
フロントエンド開発とTauriビルドでBunを使用。package.jsonのscriptsに従う。

## 開発ガイドライン
- 言われた機能のみを実装すること．勝手に機能を追加しない．