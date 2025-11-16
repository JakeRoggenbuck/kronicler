import { useEffect } from "react";
import { useNavigate, useSearchParams } from "react-router-dom";
import {
  Activity,
  Terminal,
  Zap,
  Database,
  ArrowRight,
  BarChart3,
} from "lucide-react";

const App = () => {
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();

  // Redirect to dashboard if share parameter is present
  useEffect(() => {
    const shareParam = searchParams.get("share");
    if (shareParam) {
      navigate(`/dashboard?share=${encodeURIComponent(shareParam)}`, {
        replace: true,
      });
    }
  }, [searchParams, navigate]);

  return (
    <div className="min-h-screen bg-slate-900 text-white">
      {/* Navigation Bar */}
      <nav className="border-b border-slate-700 bg-slate-900">
        <div className="max-w-4xl mx-auto px-6 py-4">
          <div className="flex items-center justify-between">
            <a
              href="/"
              className="flex items-center space-x-3 hover:opacity-80 transition-opacity"
            >
              <Activity className="w-6 h-6 text-green-500" />
              <span className="text-xl font-bold">Kronicler</span>
            </a>
            <div className="flex items-center space-x-6">
              <a
                href="https://github.com/JakeRoggenbuck/kronicler"
                className="text-gray-400 hover:text-white transition-colors text-sm"
              >
                GitHub
              </a>
              <a
                href="/dashboard"
                className="flex items-center space-x-2 bg-green-500 hover:bg-green-600 text-white px-4 py-2 rounded-lg transition-colors text-sm font-medium"
              >
                <BarChart3 className="w-4 h-4" />
                <span>Dashboard</span>
              </a>
            </div>
          </div>
        </div>
      </nav>

      <div className="max-w-4xl mx-auto px-6 py-16">
        {/* Header */}
        <div className="mb-16">
          <div className="flex items-center space-x-3 mb-3">
            <Activity className="w-8 h-8 text-green-500" />
            <h1 className="text-4xl font-bold">Kronicler</h1>
          </div>
          <p className="text-xl text-gray-400">
            Automatic performance capture and analytics for production Python
            applications
          </p>
        </div>

        {/* Quick Start */}
        <div className="mb-16">
          <h2 className="text-2xl font-semibold mb-6">Quick Start</h2>

          <div className="bg-slate-800 rounded-lg p-6 mb-6 border border-slate-700">
            <p className="text-gray-400 text-sm mb-3">Install with pip:</p>
            <code className="block bg-slate-900 px-4 py-3 rounded text-green-400 font-mono text-sm">
              pip install kronicler
            </code>
          </div>

          <div className="bg-slate-800 rounded-lg p-6 border border-slate-700">
            <p className="text-gray-400 text-sm mb-3">
              Add the decorator to your functions:
            </p>
            <pre className="bg-slate-900 px-4 py-3 rounded text-sm font-mono overflow-x-auto">
              <code>
                <span className="text-purple-400">import</span>{" "}
                <span className="text-blue-400">kronicler</span>
                {"\n\n"}
                <span className="text-gray-500">@kronicler.capture</span>
                {"\n"}
                <span className="text-purple-400">def</span>{" "}
                <span className="text-yellow-400">my_function</span>():
                {"\n"}
                {"    "}
                <span className="text-purple-400">pass</span>
              </code>
            </pre>
          </div>
        </div>

        {/* Why Kronicler */}
        <div className="mb-16">
          <h2 className="text-2xl font-semibold mb-6">Why Kronicler?</h2>
          <div className="space-y-4">
            <div className="flex items-start gap-3">
              <div className="w-1.5 h-1.5 bg-green-500 rounded-full mt-2"></div>
              <p className="text-gray-300">
                Automatic performance capturing with a single decorator
              </p>
            </div>
            <div className="flex items-start gap-3">
              <div className="w-1.5 h-1.5 bg-green-500 rounded-full mt-2"></div>
              <p className="text-gray-300">
                Custom columnar database written in Rust for efficient log
                storage
              </p>
            </div>
            <div className="flex items-start gap-3">
              <div className="w-1.5 h-1.5 bg-green-500 rounded-full mt-2"></div>
              <p className="text-gray-300">
                View mean, median, and percentile statistics for your functions
              </p>
            </div>
            <div className="flex items-start gap-3">
              <div className="w-1.5 h-1.5 bg-green-500 rounded-full mt-2"></div>
              <p className="text-gray-300">
                Works out-of-the-box without configuration
              </p>
            </div>
          </div>
        </div>

        {/* FastAPI Example */}
        <div className="mb-16">
          <h2 className="text-2xl font-semibold mb-6">FastAPI Example</h2>
          <p className="text-gray-400 mb-4 text-sm">
            Add Kronicler to your FastAPI server with just one line. The
            middleware automatically captures performance for all routes.
          </p>
          <div className="bg-slate-800 rounded-lg p-6 border border-slate-700">
            <pre className="text-sm font-mono overflow-x-auto">
              <code>
                <span className="text-purple-400">from</span>{" "}
                <span className="text-blue-400">fastapi</span>{" "}
                <span className="text-purple-400">import</span> FastAPI
                {"\n"}
                <span className="text-purple-400">import</span>{" "}
                <span className="text-blue-400">kronicler</span>
                {"\n\n"}
                app = FastAPI()
                {"\n"}
                app.add_middleware(kronicler.KroniclerMiddleware)
                {"\n\n"}
                <span className="text-gray-500"># Used only for the /logs route</span>
                {"\n"}
                DB = kronicler.Database(sync_consume=
                <span className="text-orange-400">True</span>){"\n\n"}
                <span className="text-gray-500">@app.get("/")</span>
                {"\n"}
                <span className="text-purple-400">def</span>{" "}
                <span className="text-yellow-400">read_root</span>():
                {"\n"}
                {"    "}
                <span className="text-purple-400">return</span> {"{"}
                <span className="text-green-400">"status"</span>:{" "}
                <span className="text-green-400">"success"</span>
                {"}"}
                {"\n\n"}
                <span className="text-gray-500">@app.get("/logs")</span>
                {"\n"}
                <span className="text-purple-400">def</span>{" "}
                <span className="text-yellow-400">read_logs</span>():
                {"\n"}
                {"    "}
                <span className="text-purple-400">return</span> DB.logs()
              </code>
            </pre>
          </div>
        </div>

        {/* Performance */}
        <div className="mb-16">
          <h2 className="text-2xl font-semibold mb-6">Performance</h2>
          <p className="text-gray-300 mb-4">
            Kronicler is designed to be lightweight. The custom columnar
            database provides significant performance improvements over
            traditional solutions:
          </p>
          <div className="grid grid-cols-2 gap-4">
            <div className="bg-slate-800 rounded-lg p-5 border border-slate-700">
              <div className="flex items-center mb-3">
                <Zap className="w-5 h-5 text-green-500 mr-2" />
              </div>
              <div className="text-3xl font-bold text-green-500 mb-2">
                7.71x
              </div>
              <div className="text-sm text-gray-400">
                Faster inserts than SQLite
              </div>
            </div>
            <div className="bg-slate-800 rounded-lg p-5 border border-slate-700">
              <div className="flex items-center mb-3">
                <Database className="w-5 h-5 text-green-500 mr-2" />
              </div>
              <div className="text-3xl font-bold text-green-500 mb-2">837x</div>
              <div className="text-sm text-gray-400">
                Faster mean calculation
              </div>
            </div>
          </div>
        </div>

        {/* Dashboard Link */}
        <div className="mb-16">
          <h2 className="text-2xl font-semibold mb-6">Analytics Dashboard</h2>
          <p className="text-gray-300 mb-6">
            View your performance metrics in real-time with the web dashboard.
          </p>
          <a
            href="/dashboard"
            className="inline-flex items-center space-x-2 bg-green-500 hover:bg-green-600 text-white font-medium px-6 py-3 rounded-lg transition-colors"
          >
            <span>Open Dashboard</span>
            <ArrowRight className="w-4 h-4" />
          </a>
        </div>

        {/* CLI */}
        <div className="mb-16">
          <h2 className="text-2xl font-semibold mb-6 flex items-center">
            <Terminal className="w-6 h-6 mr-2 text-green-500" />
            Command Line Interface
          </h2>
          <p className="text-gray-300 mb-4">
            Install the CLI tool to analyze logs from your terminal:
          </p>
          <div className="bg-slate-800 rounded-lg p-6 border border-slate-700">
            <code className="block bg-slate-900 px-4 py-3 rounded text-green-400 font-mono text-sm mb-4">
              cargo install kronicler
            </code>
            <code className="block bg-slate-900 px-4 py-3 rounded text-green-400 font-mono text-sm">
              kr --fetch all
            </code>
          </div>
        </div>

        {/* Links */}
        <div className="border-t border-slate-700 pt-8">
          <div className="flex gap-6 text-sm">
            <a
              href="https://github.com/JakeRoggenbuck/kronicler"
              className="text-gray-400 hover:text-white transition-colors"
            >
              GitHub
            </a>
            <a
              href="https://pypi.org/project/kronicler/"
              className="text-gray-400 hover:text-white transition-colors"
            >
              PyPI
            </a>
            <a
              href="https://crates.io/crates/kronicler"
              className="text-gray-400 hover:text-white transition-colors"
            >
              Crates.io
            </a>
          </div>
          <p className="text-gray-500 text-sm mt-4">MIT License</p>
        </div>
      </div>
    </div>
  );
};

export default App;
