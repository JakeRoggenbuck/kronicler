import {
  Activity,
  ArrowRightCircle,
  BookOpen,
  Database,
  Globe2,
  Layers,
  ShieldCheck,
  Terminal,
  Zap,
} from "lucide-react";
import TopNav from "./components/TopNav";

const About = () => {
  return (
    <div className="min-h-screen bg-slate-900 text-white">
      <TopNav />

      <main className="max-w-5xl mx-auto px-6 py-12 space-y-12">
        <section className="space-y-4">
          <div className="flex items-start gap-3">
            <Activity className="w-10 h-10 text-green-500 mt-1" />
            <div>
              <p className="text-sm uppercase tracking-wide text-green-400">
                About Kronicler
              </p>
              <h1 className="text-3xl font-bold">
                Performance analytics built for production applications
              </h1>
            </div>
          </div>
          <p className="text-lg text-gray-300 leading-relaxed">
            Automatic performance capture and analytics for production
            applications in Python using a custom columnar database written in
            Rust. Kronicler ships one dependency, works out-of-the-box, and
            captures runtime statistics like mean, median, and percentile
            outliers so you can spot sporadic slowdowns in real workloads.
          </p>
          <div className="inline-flex items-center gap-2 bg-slate-800 border border-slate-700 px-4 py-2 rounded-lg text-sm text-gray-200">
            <ShieldCheck className="w-4 h-4 text-green-400" />
            <span>
              Early in development — feedback is welcome to make Kronicler more
              useful.
            </span>
          </div>
        </section>

        <section className="space-y-4">
          <div className="flex items-center gap-2">
            <BookOpen className="w-5 h-5 text-green-400" />
            <h2 className="text-2xl font-semibold">What Kronicler includes</h2>
          </div>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div className="bg-slate-800 border border-slate-700 rounded-xl p-5 space-y-2">
              <div className="flex items-center gap-2 text-green-400">
                <BookOpen className="w-4 h-4" />
                <span className="text-sm uppercase">Python Library</span>
              </div>
              <p className="text-gray-200 font-semibold">
                `@kronicler.capture` decorator
              </p>
              <p className="text-gray-400 text-sm">
                Add a single decorator to capture runtimes to a local database
                and fetch insights without extra configuration.
              </p>
            </div>
            <div className="bg-slate-800 border border-slate-700 rounded-xl p-5 space-y-2">
              <div className="flex items-center gap-2 text-green-400">
                <Database className="w-4 h-4" />
                <span className="text-sm uppercase">Columnar Database</span>
              </div>
              <p className="text-gray-200 font-semibold">
                Rust-backed storage engine
              </p>
              <p className="text-gray-400 text-sm">
                Custom columnar database optimized for analytics, with capture
                queues and bufferpool pages designed for fast ingest and reads.
              </p>
            </div>
            <div className="bg-slate-800 border border-slate-700 rounded-xl p-5 space-y-2">
              <div className="flex items-center gap-2 text-green-400">
                <Terminal className="w-4 h-4" />
                <span className="text-sm uppercase">CLI & Crate</span>
              </div>
              <p className="text-gray-200 font-semibold">
                Inspect logs from the terminal
              </p>
              <p className="text-gray-400 text-sm">
                Ship the Rust crate or install the `kr` CLI to fetch, filter,
                and explore performance data locally.
              </p>
            </div>
            <div className="bg-slate-800 border border-slate-700 rounded-xl p-5 space-y-2">
              <div className="flex items-center gap-2 text-green-400">
                <Globe2 className="w-4 h-4" />
                <span className="text-sm uppercase">Web Dashboard</span>
              </div>
              <p className="text-gray-200 font-semibold">
                Remote analytics view
              </p>
              <p className="text-gray-400 text-sm">
                Access performance dashboards from usekronicler.com for
                real-time monitoring of production services.
              </p>
            </div>
          </div>
        </section>

        <section className="space-y-6">
          <div className="flex items-center gap-2">
            <Zap className="w-5 h-5 text-green-400" />
            <h2 className="text-2xl font-semibold">Why use Kronicler</h2>
          </div>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div className="bg-slate-800 border border-slate-700 rounded-xl p-5 space-y-3">
              <p className="text-gray-300">
                Automatic performance capturing with a single dependency and no
                config required.
              </p>
              <p className="text-gray-300">
                Lightweight by design with concurrent ingest in development and
                minimal runtime overhead.
              </p>
              <p className="text-gray-300">
                Percentile stats (mean, median, p95, p99) spotlight slow spikes
                that are hard to reproduce in test environments.
              </p>
            </div>
            <div className="bg-slate-800 border border-slate-700 rounded-xl p-5 space-y-3">
              <p className="text-gray-300">
                Rust-powered columnar storage keeps analytics fast with amortized
                constant average calculations.
              </p>
              <p className="text-gray-300">
                Works with frameworks like FastAPI through middleware to capture
                every route automatically.
              </p>
              <p className="text-gray-300">
				Extremely easy to setup and view analytics for any Python application.
              </p>
            </div>
          </div>
        </section>

        <section className="space-y-4">
          <div className="flex items-center gap-2">
            <Terminal className="w-5 h-5 text-green-400" />
            <h2 className="text-2xl font-semibold">Install & capture</h2>
          </div>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div className="bg-slate-800 border border-slate-700 rounded-xl p-5 space-y-3">
              <p className="text-gray-400 text-sm">Install with pip</p>
              <code className="block bg-slate-900 px-4 py-3 rounded text-green-400 font-mono text-sm">
                pip install kronicler
              </code>
              <p className="text-gray-500 text-xs">
                Use a virtual environment if you prefer isolated dependencies.
              </p>
            </div>
            <div className="bg-slate-800 border border-slate-700 rounded-xl p-5 space-y-3">
              <p className="text-gray-400 text-sm">Capture a function</p>
              <pre className="bg-slate-900 px-4 py-3 rounded text-sm font-mono overflow-x-auto text-gray-200">
{`import kronicler

@kronicler.capture
def my_function():
    pass`}
              </pre>
              <p className="text-gray-500 text-xs">
                Prefer FastAPI? Add `KroniclerMiddleware` to capture every route
                automatically.
              </p>
            </div>
          </div>
        </section>

        <section className="space-y-4">
          <div className="flex items-center gap-2">
            <Layers className="w-5 h-5 text-green-400" />
            <h2 className="text-2xl font-semibold">Architecture at a glance</h2>
          </div>
          <div className="bg-slate-800 border border-slate-700 rounded-xl p-5 space-y-3">
            <p className="text-gray-300">
              Kronicler&apos;s pipeline moves captured data efficiently with a custom Rust columnar database:
            </p>
            <ul className="list-disc list-inside text-gray-400 space-y-2">
              <li>
                The Python `capture` decorator pushes timing data into an ingest
                queue.
              </li>
              <li>
                The queue is consumed into dedicated columns, backed by a
                bufferpool that manages pages on disk.
              </li>
              <li>
                Supporting pieces like capture/index/row metadata keep fetches
                and updates fast for analytics queries.
              </li>
            </ul>
            <p className="text-gray-500 text-sm">
              The same engine powers the Rust crate and CLI, so you can write
              captures from multiple languages.
            </p>
          </div>
        </section>

        <section className="space-y-4">
          <div className="flex items-center gap-2">
            <Zap className="w-5 h-5 text-green-400" />
            <h2 className="text-2xl font-semibold">Performance highlights</h2>
          </div>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div className="bg-slate-800 border border-slate-700 rounded-xl p-5">
              <div className="flex items-center gap-2 mb-2">
                <Database className="w-4 h-4 text-green-400" />
                <span className="text-gray-300 text-sm">Ingest</span>
              </div>
              <div className="text-4xl font-bold text-green-500 mb-2">
                7.71x
              </div>
              <p className="text-gray-400 text-sm">
                Faster inserts than SQLite in benchmarks that log 1000 captures.
              </p>
            </div>
            <div className="bg-slate-800 border border-slate-700 rounded-xl p-5">
              <div className="flex items-center gap-2 mb-2">
                <ArrowRightCircle className="w-4 h-4 text-green-400" />
                <span className="text-gray-300 text-sm">Analytics</span>
              </div>
              <div className="text-4xl font-bold text-green-500 mb-2">837x</div>
              <p className="text-gray-400 text-sm">
                Faster mean calculation thanks to amortized constant updates on
                insert.
              </p>
            </div>
          </div>
        </section>

        <section className="space-y-4">
          <div className="flex items-center gap-2">
            <ShieldCheck className="w-5 h-5 text-green-400" />
            <h2 className="text-2xl font-semibold">Control & logging</h2>
          </div>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div className="bg-slate-800 border border-slate-700 rounded-xl p-5 space-y-3">
              <p className="text-gray-400 text-sm">
                Disable Kronicler temporarily with an environment flag:
              </p>
              <code className="block bg-slate-900 px-4 py-3 rounded text-green-400 font-mono text-sm">
                KRONICLER_ENABLED=false
              </code>
              <p className="text-gray-500 text-xs">
                The default is enabled, so you only set it when you need to pause
                capture.
              </p>
            </div>
            <div className="bg-slate-800 border border-slate-700 rounded-xl p-5 space-y-3">
              <p className="text-gray-400 text-sm">
                Rust internals use env_logger for visibility during development.
              </p>
              <code className="block bg-slate-900 px-4 py-3 rounded text-green-400 font-mono text-sm">
                RUST_LOG=info cargo run -- --fetch 0
              </code>
              <p className="text-gray-500 text-xs">
                Adjust log level as needed to debug captures and fetches.
              </p>
            </div>
          </div>
        </section>

        <section className="space-y-4">
          <div className="flex items-center gap-2">
            <Globe2 className="w-5 h-5 text-green-400" />
            <h2 className="text-2xl font-semibold">Explore more</h2>
          </div>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <a
              href="https://usekronicler.com"
              className="bg-slate-800 border border-slate-700 rounded-xl p-5 hover:border-green-500 transition-colors flex items-center justify-between"
            >
              <div>
                <p className="text-gray-200 font-semibold">
                  Hosted Dashboard
                </p>
                <p className="text-gray-400 text-sm">
                  View your production telemetry from anywhere.
                </p>
              </div>
              <ArrowRightCircle className="w-5 h-5 text-green-400" />
            </a>
            <div className="bg-slate-800 border border-slate-700 rounded-xl p-5 space-y-2">
              <p className="text-gray-200 font-semibold">Packages</p>
              <div className="flex flex-wrap gap-3 text-sm">
                <a
                  href="https://pypi.org/project/kronicler"
                  className="text-green-400 hover:text-green-300"
                >
                  PyPI
                </a>
                <a
                  href="https://crates.io/crates/kronicler"
                  className="text-green-400 hover:text-green-300"
                >
                  Crates.io
                </a>
                <a
                  href="https://github.com/JakeRoggenbuck/kronicler"
                  className="text-green-400 hover:text-green-300"
                >
                  GitHub
                </a>
              </div>
              <p className="text-gray-500 text-xs">
                Check the repository for release tags, CI status, and docs.
              </p>
            </div>
          </div>
        </section>
      </main>
    </div>
  );
};

export default About;
