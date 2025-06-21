import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

interface DashboardProps {
  version: string;
}

function Dashboard({ version }: DashboardProps) {
  const [videoPaths, setVideoPaths] = useState<Set<string>>(new Set());
  const [sleepDuration, setSleepDuration] = useState<number>(0);
  
  // イベントリスナー設定と初期値取得
  useEffect(() => {
    const setupEventListener = async () => {
      // 現在のスリープ時間を取得
      try {
        const currentDuration = await invoke<number>("get_sleep_duration");
        setSleepDuration(currentDuration);
      } catch (error) {
        console.error("Failed to get sleep duration:", error);
      }
      
      // イベントリスナー設定
      const unlisten = await listen<string>("video_path_added", (event) => {
        console.log("新しい動画パス受信:", event.payload);
        setVideoPaths(prev => new Set([...prev, event.payload]));
      });
      
      return unlisten;
    };
    
    let unlisten: (() => void) | undefined;
    
    setupEventListener().then((unlistenFn) => {
      unlisten = unlistenFn;
    });
    
    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  }, []);

  const handlePlayHighlights = async () => {
    try {
      const result = await invoke("play_highlights", { videoPaths: Array.from(videoPaths) });
      console.log("ハイライト再生結果:", result);
      // 再生後にパスをクリア
      setVideoPaths(new Set());
    } catch (error) {
      console.error("ハイライト再生エラー:", error);
    }
  };

  const handleSleepDurationChange = async (value: number) => {
    setSleepDuration(value);
    try {
      console.log(await invoke("set_sleep_duration", { duration: value }));
    } catch (error) {
      console.error("Failed to set sleep duration:", error);
    }
  };

  return (
    <div className="min-h-screen bg-gray-900 text-white p-8">
      <div className="max-w-4xl mx-auto">
        <div className="mb-8">
          <h1 className="text-4xl font-bold mb-2">RLHighlight Dashboard v{version}</h1>
          <div className="text-green-400 text-sm">● OBS Studio 接続済み</div>
        </div>
        
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          <div className="bg-gray-800 rounded-lg p-6">
            <h2 className="text-xl font-semibold mb-4">録画済み動画</h2>
            <div className="space-y-2">
              {videoPaths.size === 0 ? (
                <div className="text-gray-400 text-sm">録画なし</div>
              ) : (
                <div className="space-y-1 max-h-48 overflow-y-auto">
                  {Array.from(videoPaths).map((filename, index) => (
                    <div key={index} className="text-sm text-gray-300 bg-gray-700 px-2 py-1 rounded">
                      {filename}
                    </div>
                  ))}
                </div>
              )}
              <div className="text-xs text-gray-500 mt-2">
                {videoPaths.size > 0 && `${videoPaths.size}個の動画`}
              </div>
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
          
          <div className="bg-gray-800 rounded-lg p-6">
            <h2 className="text-xl font-semibold mb-4">設定</h2>
            <div className="space-y-4">
              <div>
                <label htmlFor="sleepDuration" className="block text-sm font-medium text-gray-300 mb-2">
                  録画遅延時間 (秒)
                </label>
                <input
                  type="number"
                  id="sleepDuration"
                  min="1"
                  max="30"
                  value={sleepDuration}
                  onChange={(e) => handleSleepDurationChange(parseInt(e.target.value))}
                  className="w-full px-4 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                />
                <div className="text-xs text-gray-500 mt-1">
                  ゴール/エピックセーブ検知後の録画開始までの遅延時間
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default Dashboard;
