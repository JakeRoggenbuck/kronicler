"""Pytest configuration for kronicler tests."""
import sys
from pathlib import Path

# Add the parent directory to Python path so we can import kronicler
# This allows tests to be run from the python/ directory or from the repo root
python_dir = Path(__file__).parent.parent
python_dir_str = str(python_dir.resolve())

# Add to path if not already there
if python_dir_str not in sys.path:
    sys.path.insert(0, python_dir_str)

# Also add the repo root in case tests are run from there
repo_root = Path(__file__).parent.parent.parent
repo_root_str = str(repo_root.resolve())
if repo_root_str not in sys.path:
    sys.path.insert(0, repo_root_str)

