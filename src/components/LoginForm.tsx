import { useState } from "react";

interface LoginFormProps {
  onConnect: (port: number, password?: string) => Promise<void>;
  isConnecting: boolean;
  connectionStatus: 'idle' | 'connecting' | 'connected' | 'error';
  errorMessage?: string;
  version?: string;
}

export default function LoginForm({ onConnect, isConnecting, connectionStatus, errorMessage, version }: LoginFormProps) {
  const [port, setPort] = useState<string>("4455");
  const [password, setPassword] = useState<string>("");

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    const portNumber = parseInt(port);
    if (isNaN(portNumber) || portNumber < 1 || portNumber > 65535) {
      return;
    }
    await onConnect(portNumber, password || undefined);
  };

  const getStatusColor = () => {
    switch (connectionStatus) {
      case 'connected': return 'text-green-500';
      case 'error': return 'text-red-500';
      case 'connecting': return 'text-blue-500';
      default: return 'text-gray-500';
    }
  };

  const getStatusText = () => {
    switch (connectionStatus) {
      case 'connected': return '接続済み';
      case 'error': return 'エラー';
      case 'connecting': return '接続中...';
      default: return '未接続';
    }
  };

  return (
    <div className="min-h-screen bg-gray-900 flex items-center justify-center p-4">
      <div className="bg-gray-800 rounded-lg shadow-xl p-8 w-full max-w-md">
        <div className="text-center mb-8">
          <h1 className="text-3xl font-bold text-white mb-2">RLHighlight v{version}</h1>
          <p className="text-gray-400">OBS Studio接続</p>
        </div>

        <div className="mb-6">
          <div className={`text-center text-sm font-medium ${getStatusColor()}`}>
            ● {getStatusText()}
          </div>
          {errorMessage && (
            <div className="mt-2 text-red-400 text-sm text-center">
              {errorMessage}
            </div>
          )}
        </div>

        <form onSubmit={handleSubmit} className="space-y-6">
          <div>
            <label htmlFor="port" className="block text-sm font-medium text-gray-300 mb-2">
              ポート番号 *
            </label>
            <input
              type="number"
              id="port"
              required
              min="1"
              max="65535"
              value={port}
              onChange={(e) => setPort(e.target.value)}
              className="w-full px-4 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              placeholder="4455"
              disabled={isConnecting || connectionStatus === 'connected'}
            />
          </div>

          <div>
            <label htmlFor="password" className="block text-sm font-medium text-gray-300 mb-2">
              パスワード (任意)
            </label>
            <input
              type="password"
              id="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              className="w-full px-4 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              placeholder="パスワードを入力"
              disabled={isConnecting || connectionStatus === 'connected'}
            />
          </div>

          <button
            type="submit"
            disabled={isConnecting || connectionStatus === 'connected'}
            className="w-full bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white font-medium py-2 px-4 rounded-lg transition-colors duration-200 flex items-center justify-center"
          >
            {isConnecting ? (
              <>
                <svg className="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                  <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                  <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
                接続中...
              </>
            ) : connectionStatus === 'connected' ? (
              '接続済み'
            ) : (
              'OBSに接続'
            )}
          </button>
        </form>

        <div className="mt-6 text-center text-xs text-gray-500">
          OBS Studio が起動し、WebSocketサーバーが有効になっていることを確認してください
        </div>
      </div>
    </div>
  );
}
