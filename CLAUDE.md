# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## プロジェクト概要

Rocket League（RL）大会用のハイライト自動作成・再生システムのTauriデスクトップアプリケーション。BakkesModプラグインからのUDPデータを受信し、OBS Studioと連携してリプレイバッファの録画・再生を自動化する。

## アーキテクチャ

### フロントエンド・バックエンド構成
- **フロントエンド**: React + TypeScript + Vite + Tailwind CSS（`src/`）
- **バックエンド**: Rust + Tauri v2（`src-tauri/src/`）
- **ビルドシステム**: Tauri v2 + Bun
- **UIフレームワーク**: React 18 + Tailwind CSS v4

### 主要ファイル構成
- `src/App.tsx`: メインアプリケーション（接続状態管理・ダッシュボード）
- `src/components/LoginForm.tsx`: OBS接続フォーム
- `src-tauri/src/lib.rs`: Tauriメイン・コマンド・状態管理
- `src-tauri/src/obs.rs`: OBS WebSocket制御
- `src-tauri/src/udp.rs`: UDP受信サーバー
- `src-tauri/src/vlc_manager.rs`: VLCファイルパス管理
- `src-tauri/src/mugi_schema.rs`: コマンドスキーマ解析

### 非同期イベント処理アーキテクチャ
Tauriランタイム内で並行実行される非同期タスク：

1. **UDP受信サーバー** (`udp.rs`) - ポート12345でBakkesModコマンド受信
2. **OBS制御システム** (`obs.rs`) - WebSocket経由でOBS Studio制御（localhost:4455）
3. **VLCファイル管理** (`vlc_manager.rs`) - リプレイファイルパスのキューイング
4. **Tauriコマンド** (`lib.rs`) - フロントエンドとの通信インターフェース

### イベントフロー
1. BakkesModがUDPでゲームイベント送信（"scored"/"epicSave"）
2. 3秒遅延後にOBSリプレイバッファ保存トリガー
3. OBSイベントストリームでファイルパス取得→VlcManagerキューに蓄積
4. "dbg"コマンドでOBS VLCソース経由で全動画再生
5. フロントエンドからTauriコマンド経由でOBS接続・操作制御

## 開発コマンド

### 開発環境起動
```bash
bun.exe tauri dev
```

### ビルド
```bash
# フロントエンド＋バックエンドビルド
bun run build
bun tauri build

# Rustバックエンドのみ
cd src-tauri && cargo build
```

### Lint
```bash
# ./src-tauriフォルダ内で
cargo.exe clippy --fix
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
- `lib.rs`の`run()`でTauriアプリ初期化・コマンドハンドラー登録
- `AppState`で接続情報とシステム動作状態を管理
- `console_subscriber::init()`でtokioトレーシング有効化
- Tauriコマンド: `connect_obs`, `send_udp_command`

### フロントエンド状態管理
- React useState: 接続状態（idle/connecting/connected/error）
- OBS接続後は自動的にダッシュボード画面に遷移
- リアルタイムシステム状態表示（UDP/VLC/リプレイバッファ）

### 外部依存関係
- **OBS Studio**: WebSocketサーバー必須（ポート4455、パスワード対応）
- **カスタムOBWS**: `fix-VlcSource`ブランチ使用（VLC制御機能）
- **BakkesMod**: Rocket Leagueプラグイン（UDPイベント送信）
- **sccache**: Rustコンパイル高速化（Windows環境）

### 状態管理とライフサイクル
- `AppState`: OBS接続情報・システム動作フラグをArc<Mutex>で管理
- システム起動時: `run_main_system()`で無限ループによるUDPコマンド待機
- VLCファイルパス: 静的`LazyLock<Mutex<Vec<PathBuf>>>`でグローバル蓄積

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

## システム注意事項
- ソースコードの実態はwindows上にあるため，bunやcargoコマンドを使用する場合はbun.exeやcargo.exeのようにする．