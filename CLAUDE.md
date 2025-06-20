# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## プロジェクト概要

Rocket League（RL）大会用のハイライト自動作成・再生システムのTauriデスクトップアプリケーション。BakkesModプラグインからのUDPデータを受信し、OBS Studioと連携してリプレイバッファの録画・再生を自動化する。

バージョン: `0.2.0` - プロダクト名: `RL_Highlight`

## アーキテクチャ

### フロントエンド・バックエンド構成
- **フロントエンド**: React 18 + TypeScript + Vite + Tailwind CSS v4（`src/`）
- **バックエンド**: Rust 2024 Edition + Tauri v2（`src-tauri/src/`）
- **ビルドシステム**: Tauri v2 + Bun
- **自動更新**: GitHub Releases連携

### 主要ファイル構成
- `src/App.tsx`: メインアプリケーション（接続状態管理・自動画面遷移）
- `src/components/LoginForm.tsx`: OBS接続フォーム（ポート・パスワード設定）
- `src/components/Dashboard.tsx`: メインダッシュボード（動画管理・設定UI）
- `src-tauri/src/lib.rs`: Tauriコマンド・状態管理・自動更新
- `src-tauri/src/obs.rs`: OBS WebSocket制御・VLC連携
- `src-tauri/src/udp.rs`: UDP受信サーバー（ポート12345）
- `src-tauri/src/vlc_manager.rs`: 動画ファイルパス管理・イベント送信
- `src-tauri/src/mugi_schema.rs`: BakkesModコマンド解析（26種類対応）

### 非同期イベント処理アーキテクチャ
Tauriランタイム内で並行実行される非同期タスク：

1. **UDP受信サーバー** (`udp.rs`) - ポート12345でBakkesModコマンド受信
2. **OBS制御システム** (`obs.rs`) - WebSocket経由でOBS Studio制御（localhost:4455）
3. **VLCファイル管理** (`vlc_manager.rs`) - リプレイファイルパス管理・フロントエンド通知
4. **Tauriコマンド** (`lib.rs`) - フロントエンドとの通信インターフェース
5. **自動更新システム** - GitHub Releases監視・ダウンロード・インストール

### イベントフロー
1. BakkesModがUDPでゲームイベント送信（"scored"/"epicSave"）  
2. **設定可能な遅延時間**（1-30秒、デフォルト3秒）後にOBSリプレイバッファ保存
3. OBS `ReplayBufferSaved`イベント → VlcManagerがファイルパス受信
4. フロントエンドに`video_path_added`イベント送信（ファイル名のみ）
5. ユーザーが「ハイライト再生」クリック → VLCソース経由で動画再生

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
- `lib.rs`の`run()`でTauriアプリ初期化・コマンドハンドラー登録・自動更新
- `AppState`で接続情報・システム動作状態・録画遅延時間を管理
- `console_subscriber::init()`でtokioトレーシング有効化
- **Tauriコマンド**: `connect_obs`, `play_highlights`, `set_sleep_duration`, `get_sleep_duration`

### 状態管理とライフサイクル
- **`AppState`構造体**:
  - `obs_connection_info: Arc<Mutex<Option<(String, u16, Option<String>)>>>`
  - `is_system_running: Arc<Mutex<bool>>`
  - `sleep_duration_sec: Arc<RwLock<u64>>` （1-30秒、デフォルト3秒）
- システム起動時: `run_main_system()`で無限ループによるUDPコマンド待機
- スレッドセーフな状態共有: `Arc<Mutex>`, `Arc<RwLock>`使用

### フロントエンド状態管理
- **接続状態**: `idle/connecting/connected/error`の4状態
- **自動画面遷移**: OBS接続成功後にダッシュボードへ自動切り替え
- **動画管理**: `Set<string>`でファイル名重複除去・リアルタイム表示
- **設定UI**: ダッシュボードで録画遅延時間をリアルタイム変更可能

### 外部依存関係
- **OBS Studio**: WebSocketサーバー必須（ポート4455、パスワード対応）
- **カスタムOBWS**: `fix-VlcSource`ブランチ使用（VLC制御機能拡張）
  - リポジトリ: `https://github.com/Kazuryu0907/obws`
  - VLCソース名: `"RL_REPLAY_VLC_SOURCE"`
- **BakkesMod**: Rocket Leagueプラグイン（UDPイベント送信、ポート12345）

### VLC動画管理システム
- **ファイルパス処理**: OBS `ReplayBufferSaved`イベントからフルパス取得
- **フロントエンド通知**: ファイル名のみを`video_path_added`イベントで送信
- **再生システム**: 複数動画を順次再生、再生後にパスクリア
- **重複排除**: フロントエンドでSet型による自動重複除去

### デバッグ・テストツール
- `udp_cli.py`: 手動UDPコマンド送信（継続利用）
- **サポートコマンド**: `scored`, `epicSave`, `dbg`など26種類
- **テスト関数**: `cargo test test_all` で動作確認可能

## 設定要件

### 必須サービス
- **OBS Studio**: WebSocketサーバー有効化（ポート4455）
- **BakkesMod**: Rocket League起動時にUDPイベント送信
- **VLCソース**: OBS内で`RL_REPLAY_VLC_SOURCE`という名前のVLCソース

### アプリケーション設定
- **ウィンドウサイズ**: 800x600
- **自動更新**: GitHub Releases連携で自動チェック・ダウンロード
- **録画遅延**: 1-30秒の範囲で設定可能（ダッシュボードのUI）

### Bunコマンド使用
フロントエンド開発とTauriビルドでBunを使用。package.jsonのscriptsに従う。

## UI/UX設計

### ログイン画面（LoginForm）
- OBSポート番号入力（デフォルト4455、1-65535範囲）
- パスワード入力（任意）
- 接続状態表示（未接続/接続中/接続済み/エラー）

### ダッシュボード（Dashboard）
- **3カラムレイアウト**:
  1. **録画済み動画**: ファイル名リスト・件数表示
  2. **操作**: ハイライト再生ボタン・システム状態
  3. **設定**: 録画遅延時間設定（1-30秒）

## 開発ガイドライン
- 言われた機能のみを実装すること．勝手に機能を追加しない．
- 新機能追加時は状態管理の一貫性を保つ（AppState, React useState）
- エラーハンドリングを適切に実装し、ユーザーフレンドリーなメッセージを表示

## システム注意事項
- **環境**: Windows上のWSL2環境
- **コマンド実行**: `bun.exe`, `cargo.exe`のように拡張子付きで実行
- **並行処理**: 複数の非同期タスクが並行実行されるため、状態管理に注意
- **メモリ管理**: Arc<Mutex>, Arc<RwLock>による適切な並行アクセス制御