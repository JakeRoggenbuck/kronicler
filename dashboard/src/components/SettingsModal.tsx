import { useState } from "react";
import { Settings } from "lucide-react";

interface SettingsModalProps {
  isOpen: boolean;
  onClose: () => void;
  currentApiUrl: string;
  onSave: (newApiUrl: string) => void;
}

const SettingsModal = ({
  isOpen,
  onClose,
  currentApiUrl,
  onSave,
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

  if (!isOpen) return null;

  return (
    <div
      className="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
      onClick={onClose}
    >
      <div
        className="bg-slate-800 rounded-xl p-6 border border-slate-700 max-w-md w-full mx-4"
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

export default SettingsModal;
