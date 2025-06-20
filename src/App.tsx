import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import LoginForm from "./components/LoginForm";
import Dashboard from "./components/Dashboard";
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

  // 接続が完了している場合はダッシュボードを表示
  if (connectionStatus === 'connected') {
    return <Dashboard />;
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
