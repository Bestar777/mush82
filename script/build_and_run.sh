#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
export PATH="/opt/homebrew/opt/rustup/bin:$HOME/.cargo/bin:$PATH"
mode="${1:-run}"
case "$mode" in
  run|--build-only|--verify|--debug|--logs) ;;
  *) echo "Usage: $0 [--build-only|--verify|--debug|--logs]" >&2; exit 2 ;;
esac
app="$PWD/build/Mush8.app"
# Do not replace the executable beneath an active game or its launcher.
if pgrep -f "$app/Contents/MacOS/" >/dev/null; then
  echo "Quit Mush8 before rebuilding. Game data and saves are retained." >&2
  exit 1
fi
./make-app.sh
codesign --verify --deep --strict "$app"
case "$mode" in
  --build-only) ;;
  --debug) lldb -- "$app/Contents/MacOS/Launcher" ;;
  --logs) open "$app"; tail -F "$HOME/Library/Logs/Mush8.log" ;;
  --verify) open "$app"; sleep 2; pgrep -f "$app/Contents/MacOS/Launcher" >/dev/null ;;
  run) open "$app" ;;
esac
