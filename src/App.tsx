import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import LoginForm from "./components/LoginForm";
import "./App.css";

type ConnectionStatus = 'idle' | 'connecting' | 'connected' | 'error';

function App() {
  const [connectionStatus, setConnectionStatus] = useState<ConnectionStatus>('idle');
  const [isConnecting, setIsConnecting] = useState(false);
  const [errorMessage, setErrorMessage] = useState<string>("");
  

  const handleConnect = async (port: number, password?: string) => {
    setIsConnecting(true);
    setConnectionStatus('connecting');
    setErrorMessage("");

    try {
      const result = await invoke("connect_obs", { 
        host: "localhost", 
        port, 
        password: password || null 
      });
      
      console.log("OBS connection result:", result);
      setConnectionStatus('connected');
    } catch (error) {
      setConnectionStatus('error');
      setErrorMessage(error as string || "接続に失敗しました");
      console.error("OBS connection error:", error);
    } finally {
      setIsConnecting(false);
    }
  };

  const handlePlayHighlights = async () => {
    try {
      // DBGコマンドを送信してハイライト再生をトリガー
      await invoke("send_udp_command", { command: "dbg" });
      console.log("ハイライト再生コマンドを送信しました");
    } catch (error) {
      console.error("ハイライト再生エラー:", error);
    }
  };

  // 接続が完了している場合はメイン画面を表示
  if (connectionStatus === 'connected') {
    return (
      <div className="min-h-screen bg-gray-900 text-white p-8">
        <div className="max-w-4xl mx-auto">
          <div className="mb-8">
            <h1 className="text-4xl font-bold mb-2">RL Replay Dashboard</h1>
            <div className="text-green-400 text-sm">● OBS Studio 接続済み</div>
          </div>
          
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div className="bg-gray-800 rounded-lg p-6">
              <h2 className="text-xl font-semibold mb-4">システム状態</h2>
              <div className="space-y-2 text-sm">
                <div>UDP サーバー: <span className="text-blue-400">ポート 12345 待機中</span></div>
                <div>VLC マネージャー: <span className="text-blue-400">待機中</span></div>
                <div>リプレイバッファ: <span className="text-blue-400">準備完了</span></div>
              </div>
            </div>
            
            <div className="bg-gray-800 rounded-lg p-6">
              <h2 className="text-xl font-semibold mb-4">操作</h2>
              <div className="space-y-4">
                <button
                  onClick={handlePlayHighlights}
                  className="w-full bg-green-600 hover:bg-green-700 text-white font-medium py-3 px-4 rounded-lg transition-colors duration-200"
                >
                  ハイライト再生
                </button>
                <div className="space-y-2 text-sm text-gray-300">
                  <div>• BakkesModでゴール/エピックセーブ時に自動録画</div>
                  <div>• UDPポート12345でコマンド受信中</div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    );
  }

  // 未接続の場合はログイン画面を表示
  return (
    <LoginForm
      onConnect={handleConnect}
      isConnecting={isConnecting}
      connectionStatus={connectionStatus}
      errorMessage={errorMessage}
    />
  );
}

export default App;
