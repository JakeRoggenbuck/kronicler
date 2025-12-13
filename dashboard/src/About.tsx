import {
  Activity,
  ArrowRight,
  Database,
  Globe2,
  Info,
  Terminal,
  Zap,
} from "lucide-react";

const About = () => {
  return (
    <div className="min-h-screen bg-slate-900 text-white">
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
                href="/about"
                className="text-gray-200 hover:text-white transition-colors text-sm font-medium"
              >
                About
              </a>
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
                <Globe2 className="w-4 h-4" />
                <span>Dashboard</span>
              </a>
            </div>
          </div>
        </div>
      </nav>

      <div className="max-w-4xl mx-auto px-6 py-16 space-y-14">
        <header className="space-y-4">
          <div className="flex items-center space-x-3">
            <Info className="w-8 h-8 text-green-500" />
            <div>
              <p className="text-sm uppercase tracking-wide text-green-400">
                Performance Analytics
              </p>
              <h1 className="text-4xl font-bold">About Kronicler</h1>
            </div>
          </div>
          <p className="text-lg text-gray-300 leading-relaxed">
            Automatic performance capture and analytics for production
            applications in Python, powered by a custom columnar database
            written in Rust. Kronicler is designed to be lightweight, concurrent
            (in development), and ready to use out-of-the-box with a single
            Python dependency.
          </p>
          <div className="flex flex-wrap gap-3 text-sm text-gray-400">
            <span className="bg-slate-800 px-3 py-1 rounded-full border border-slate-700">
              Python decorator capture
            </span>
            <span className="bg-slate-800 px-3 py-1 rounded-full border border-slate-700">
              Rust columnar database
            </span>
            <span className="bg-slate-800 px-3 py-1 rounded-full border border-slate-700">
              Analytics CLI + Web
            </span>
          </div>
        </header>

        <section>
          <h2 className="text-2xl font-semibold mb-6">Why use Kronicler</h2>
          <div className="grid md:grid-cols-2 gap-4">
            <div className="bg-slate-800 rounded-lg p-5 border border-slate-700 flex items-start gap-3">
              <Zap className="w-5 h-5 text-green-500 mt-0.5" />
              <div>
                <h3 className="font-semibold mb-2">Automatic capture</h3>
                <p className="text-gray-400 text-sm">
                  Decorate functions with{" "}
                  <code className="text-green-400">@kronicler.capture</code> to
                  log performance instantly.
                </p>
              </div>
            </div>
            <div className="bg-slate-800 rounded-lg p-5 border border-slate-700 flex items-start gap-3">
              <Database className="w-5 h-5 text-green-500 mt-0.5" />
              <div>
                <h3 className="font-semibold mb-2">Built for speed</h3>
                <p className="text-gray-400 text-sm">
                  Custom Rust columnar database delivers faster inserts and
                  analytics than general-purpose stores.
                </p>
              </div>
            </div>
            <div className="bg-slate-800 rounded-lg p-5 border border-slate-700 flex items-start gap-3">
              <Activity className="w-5 h-5 text-green-500 mt-0.5" />
              <div>
                <h3 className="font-semibold mb-2">Actionable stats</h3>
                <p className="text-gray-400 text-sm">
                  Track mean, median, and percentile performance to surface
                  occasionally slow functions.
                </p>
              </div>
            </div>
            <div className="bg-slate-800 rounded-lg p-5 border border-slate-700 flex items-start gap-3">
              <Terminal className="w-5 h-5 text-green-500 mt-0.5" />
              <div>
                <h3 className="font-semibold mb-2">Ready to ship</h3>
                <p className="text-gray-400 text-sm">
                  Works without configuration, with CLI and web analytics for
                  viewing captured logs.
                </p>
              </div>
            </div>
          </div>
        </section>

        <section>
          <h2 className="text-2xl font-semibold mb-6">
            What makes up Kronicler
          </h2>
          <div className="space-y-4">
            <div className="bg-slate-800 rounded-lg p-6 border border-slate-700">
              <div className="flex items-center gap-3 mb-3">
                <Zap className="w-5 h-5 text-green-500" />
                <h3 className="font-semibold text-lg">Capture</h3>
              </div>
              <p className="text-gray-400 text-sm mb-4">
                The Python decorator{" "}
                <code className="text-green-400">@kronicler.capture</code>{" "}
                records function runtimes, shipping them directly into the
                database.
              </p>
              <div className="bg-slate-900 rounded-lg p-4 border border-slate-800 text-sm font-mono text-gray-200 overflow-x-auto">
                <span className="text-purple-400">import</span>{" "}
                <span className="text-blue-400">kronicler</span>
                <br />
                <br />
                <span className="text-gray-500">@kronicler.capture</span>
                <br />
                <span className="text-purple-400">def</span>{" "}
                <span className="text-yellow-400">process</span>():
                <br />
                <span className="pl-4">
                  <span className="text-purple-400">return</span>{" "}
                  <span className="text-green-400">"ok"</span>
                </span>
              </div>
            </div>

            <div className="bg-slate-800 rounded-lg p-6 border border-slate-700 space-y-3">
              <div className="flex items-center gap-3">
                <Database className="w-5 h-5 text-green-500" />
                <h3 className="font-semibold text-lg">Columnar database</h3>
              </div>
              <p className="text-gray-400 text-sm">
                Logs flow from the capture decorator to a queue, then into
                columns managed by a Rust buffer pool. Each field is stored
                column-wise for faster inserts and analytics across large
                datasets.
              </p>
            </div>

            <div className="bg-slate-800 rounded-lg p-6 border border-slate-700">
              <div className="flex items-center gap-3 mb-3">
                <Globe2 className="w-5 h-5 text-green-500" />
                <h3 className="font-semibold text-lg">Interfaces</h3>
              </div>
              <ul className="space-y-2 text-sm text-gray-300">
                <li className="flex items-start gap-2">
                  <span className="text-green-500">•</span>
                  <span>Python library for decorators and manual capture.</span>
                </li>
                <li className="flex items-start gap-2">
                  <span className="text-green-500">•</span>
                  <span>Rust library to access the database directly.</span>
                </li>
                <li className="flex items-start gap-2">
                  <span className="text-green-500">•</span>
                  <span>CLI and prototype web dashboard for analytics.</span>
                </li>
              </ul>
            </div>
          </div>
        </section>

        <section className="grid md:grid-cols-2 gap-6 items-center">
          <div className="space-y-3">
            <h2 className="text-2xl font-semibold">Getting started</h2>
            <p className="text-gray-300 text-sm">
              Install the Python package and decorate the functions you want to
              observe. Use the CLI or this dashboard to explore runtime
              statistics and percentiles in production environments.
            </p>
            <div className="flex flex-wrap gap-3">
              <code className="bg-slate-800 px-4 py-2 rounded text-green-400 font-mono text-sm border border-slate-700">
                pip install kronicler
              </code>
              <code className="bg-slate-800 px-4 py-2 rounded text-green-400 font-mono text-sm border border-slate-700">
                cargo install kronicler
              </code>
            </div>
          </div>
          <div className="bg-slate-800 rounded-lg p-5 border border-slate-700">
            <div className="flex items-center gap-2 mb-2 text-sm text-gray-400">
              <ArrowRight className="w-4 h-4 text-green-500" />
              <span>Performance insights</span>
            </div>
            <p className="text-gray-300 text-sm mb-3">
              Spot functions that occasionally run slower than expected by
              comparing mean, median, and high-percentile timings over time.
            </p>
            <p className="text-gray-500 text-xs">
              Concurrency support is in development. Follow progress in issue
              #123.
            </p>
          </div>
        </section>

        <section className="border-t border-slate-800 pt-10">
          <div className="flex flex-wrap gap-6 text-sm">
            <a
              href="https://usekronicler.com"
              className="text-gray-400 hover:text-white transition-colors"
            >
              UseKronicler.com
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
            <a
              href="https://github.com/JakeRoggenbuck/kronicler"
              className="text-gray-400 hover:text-white transition-colors"
            >
              GitHub
            </a>
          </div>
          <p className="text-gray-500 text-sm mt-4">
            Kronicler is early in development and feedback is welcome!
          </p>
        </section>
      </div>
    </div>
  );
};

export default About;
