import { useState } from "react";
import { Settings, Trash2, Clock } from "lucide-react";
import type { UrlHistoryItem } from "../types";

interface SettingsModalProps {
  isOpen: boolean;
  onClose: () => void;
  currentApiUrl: string;
  onSave: (newApiUrl: string) => void;
  urlHistory: UrlHistoryItem[];
  onRemoveFromHistory: (id: string) => void;
  truncateFunctionNames: boolean;
  onTruncateToggle: (truncate: boolean) => void;
}

const SettingsModal = ({
  isOpen,
  onClose,
  currentApiUrl,
  onSave,
  urlHistory,
  onRemoveFromHistory,
  truncateFunctionNames,
  onTruncateToggle,
}: SettingsModalProps) => {
  const [tempApiUrl, setTempApiUrl] = useState(currentApiUrl);

  const handleSave = () => {
    onSave(tempApiUrl);
    onClose();
  };

  const handleCancel = () => {
    setTempApiUrl(currentApiUrl);
    onClose();
  };

  const handleUrlSelect = (url: string) => {
    setTempApiUrl(url);
  };

  const formatTimestamp = (timestamp: number) => {
    const date = new Date(timestamp);
    const now = new Date();
    const diffInHours = Math.floor(
      (now.getTime() - date.getTime()) / (1000 * 60 * 60),
    );

    if (diffInHours < 1) return "Just now";
    if (diffInHours < 24) return `${diffInHours}h ago`;
    if (diffInHours < 168) return `${Math.floor(diffInHours / 24)}d ago`;
    return date.toLocaleDateString();
  };

  if (!isOpen) return null;

  return (
    <div
      className="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
      onClick={onClose}
    >
      <div
        className="bg-slate-800 rounded-xl p-6 border border-slate-700 max-w-lg w-full mx-4"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex items-center justify-between mb-4">
          <h3 className="text-xl font-semibold flex items-center">
            <Settings className="w-5 h-5 mr-2 text-green-500" />
            Settings
          </h3>
          <button onClick={onClose} className="text-gray-400 hover:text-white">
            ✕
          </button>
        </div>

        <div className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-400 mb-2">
              API Endpoint URL
            </label>
            <input
              type="text"
              value={tempApiUrl}
              onChange={(e) => setTempApiUrl(e.target.value)}
              className="w-full bg-slate-700 text-white rounded-lg px-3 py-2 border border-slate-600 focus:border-green-500 focus:outline-none"
              placeholder="https://api.algoboard.org/logs"
            />
            <p className="text-xs text-gray-500 mt-2">
              Example: https://api.algoboard.org/logs
            </p>
            <p className="text-xs text-gray-400 mt-1">
              If you're running Kronicler on your own server, enter your
              endpoint URL here (e.g., http://localhost:8000/logs)
            </p>
          </div>

          {urlHistory.length > 0 && (
            <div>
              <label className="block text-sm font-medium text-gray-400 mb-2">
                <Clock className="w-4 h-4 inline mr-1" />
                URL History
              </label>
              <div className="space-y-2 max-h-40 overflow-y-auto">
                {urlHistory.map((item) => (
                  <div
                    key={item.id}
                    className="flex items-center justify-between bg-slate-700 rounded-lg px-3 py-2 border border-slate-600 hover:border-slate-500 transition-colors"
                  >
                    <div className="flex-1 min-w-0">
                      <button
                        onClick={() => handleUrlSelect(item.url)}
                        className="text-left text-sm text-white hover:text-green-400 transition-colors truncate w-full"
                        title={item.url}
                      >
                        {item.url}
                      </button>
                      <p className="text-xs text-gray-500 mt-1">
                        {formatTimestamp(item.timestamp)}
                      </p>
                    </div>
                    <button
                      onClick={() => onRemoveFromHistory(item.id)}
                      className="ml-2 p-1 text-gray-400 hover:text-red-400 transition-colors"
                      title="Remove from history"
                    >
                      <Trash2 className="w-4 h-4" />
                    </button>
                  </div>
                ))}
              </div>
            </div>
          )}

          <div>
            <label className="block text-sm font-medium text-gray-400 mb-3">
              Display Options
            </label>
            <div className="flex items-center justify-between bg-slate-700 rounded-lg px-3 py-2 border border-slate-600">
              <div>
                <p className="text-sm text-white">Truncate Function Names</p>
                <p className="text-xs text-gray-400">
                  Show only first two path segments (e.g., /api/services)
                </p>
              </div>
              <button
                onClick={() => onTruncateToggle(!truncateFunctionNames)}
                className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
                  truncateFunctionNames ? "bg-green-500" : "bg-slate-600"
                }`}
              >
                <span
                  className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                    truncateFunctionNames ? "translate-x-6" : "translate-x-1"
                  }`}
                />
              </button>
            </div>
          </div>

          <div className="flex space-x-3 pt-2">
            <button
              onClick={handleSave}
              className="flex-1 bg-green-500 hover:bg-green-600 text-white px-4 py-2 rounded-lg transition-colors"
            >
              Save & Refresh
            </button>
            <button
              onClick={handleCancel}
              className="flex-1 bg-slate-700 hover:bg-slate-600 text-white px-4 py-2 rounded-lg transition-colors"
            >
              Cancel
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};

// SettingsModal component with truncate toggle
export default SettingsModal;
