import { Activity, BarChart3, Home, Info, Github } from "lucide-react";
import { Link, useLocation } from "react-router-dom";

const TopNav = () => {
  const location = useLocation();

  const isActive = (path: string) => {
    if (path === "/") {
      return location.pathname === "/";
    }
    return location.pathname.startsWith(path);
  };

  const linkClasses = (path: string) =>
    `flex items-center gap-2 px-3 py-2 rounded-lg text-sm transition-colors ${
      isActive(path)
        ? "bg-slate-800 text-white border border-slate-700"
        : "text-gray-400 hover:text-white"
    }`;

  return (
    <nav className="border-b border-slate-700 bg-slate-900">
      <div className="max-w-5xl mx-auto px-6 py-4 flex items-center justify-between">
        <Link
          to="/"
          className="flex items-center space-x-3 hover:opacity-80 transition-opacity"
        >
          <Activity className="w-6 h-6 text-green-500" />
          <span className="text-xl font-bold">Kronicler</span>
        </Link>

        <div className="flex items-center space-x-2">
          <Link to="/" className={linkClasses("/")}>
            <Home className="w-4 h-4" />
            <span>Home</span>
          </Link>
          <Link to="/dashboard" className={linkClasses("/dashboard")}>
            <BarChart3 className="w-4 h-4" />
            <span>Dashboard</span>
          </Link>
          <Link to="/about" className={linkClasses("/about")}>
            <Info className="w-4 h-4" />
            <span>About</span>
          </Link>
          <a
            href="https://github.com/JakeRoggenbuck/kronicler"
            target="_blank"
            rel="noreferrer"
            className="flex items-center gap-2 px-3 py-2 rounded-lg text-sm bg-green-500 hover:bg-green-600 text-white transition-colors"
          >
            <Github className="w-4 h-4" />
            <span>GitHub</span>
          </a>
        </div>
      </div>
    </nav>
  );
};

export default TopNav;
