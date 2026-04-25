#!/usr/bin/env bash
set -euo pipefail

AGENTS_DIR_NAME=".agents"
BACKUP_DIR_NAME=".backup"
BACKUP_LATEST_FILE="LATEST"

ROOT=""
SNAPSHOT=""
DRY_RUN=0

usage() {
  cat <<'USAGE'
Usage:
  restore_agent.sh [--root PATH] [--snapshot NAME] [--dry-run]
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --root)
      ROOT="$2"
      shift 2
      ;;
    --snapshot)
      SNAPSHOT="$2"
      shift 2
      ;;
    --dry-run)
      DRY_RUN=1
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "error: unknown option: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

infer_repo_root() {
  local probe
  if [[ -n "$ROOT" ]]; then
    probe="$(cd "$ROOT" && pwd -P)"
    echo "$probe"
    return 0
  fi

  probe="$(cd "$(dirname "$0")" && pwd -P)"
  while [[ "$probe" != "/" ]]; do
    if [[ -d "$probe/$AGENTS_DIR_NAME/scripts" ]]; then
      echo "$probe"
      return 0
    fi
    probe="$(dirname "$probe")"
  done

  probe="$(pwd -P)"
  while [[ "$probe" != "/" ]]; do
    if [[ -d "$probe/$AGENTS_DIR_NAME/scripts" ]]; then
      echo "$probe"
      return 0
    fi
    probe="$(dirname "$probe")"
  done

  echo "error: could not infer repository root. Pass --root explicitly." >&2
  exit 2
}

REPO_ROOT="$(infer_repo_root)"
BACKUP_BASE="$REPO_ROOT/$AGENTS_DIR_NAME/$BACKUP_DIR_NAME"

if [[ -z "$SNAPSHOT" ]]; then
  if [[ ! -f "$BACKUP_BASE/$BACKUP_LATEST_FILE" ]]; then
    echo "error: no latest backup snapshot found." >&2
    exit 2
  fi
  SNAPSHOT="$(tr -d '\n' < "$BACKUP_BASE/$BACKUP_LATEST_FILE")"
fi

BACKUP_ROOT="$BACKUP_BASE/$SNAPSHOT"
if [[ ! -d "$BACKUP_ROOT" ]]; then
  echo "error: backup snapshot not found: $BACKUP_ROOT" >&2
  exit 2
fi

MANIFEST_ENV="$BACKUP_ROOT/manifest.env"
if [[ ! -f "$MANIFEST_ENV" ]]; then
  echo "error: manifest not found: $MANIFEST_ENV" >&2
  exit 2
fi

# shellcheck disable=SC1090
source "$MANIFEST_ENV"

declare -A OVERWRITTEN_SET

load_overwritten_set() {
  local file="$BACKUP_ROOT/overwritten_targets.txt"
  [[ -f "$file" ]] || return 0
  while IFS= read -r line || [[ -n "$line" ]]; do
    [[ -n "$line" ]] || continue
    OVERWRITTEN_SET["$line"]=1
  done < "$file"
}

restore_tree() {
  local src_root="$1"
  [[ -d "$src_root" ]] || return 0
  if [[ $DRY_RUN -eq 1 ]]; then
    return 0
  fi
  cp -a "$src_root/." "$REPO_ROOT/"
}

remove_created_files() {
  local file="$BACKUP_ROOT/created_files.txt"
  [[ -f "$file" ]] || return 0
  while IFS= read -r rel || [[ -n "$rel" ]]; do
    [[ -n "$rel" ]] || continue
    if [[ -n "${OVERWRITTEN_SET[$rel]:-}" ]]; then
      continue
    fi
    if [[ -e "$REPO_ROOT/$rel" ]]; then
      [[ $DRY_RUN -eq 1 ]] || rm -f "$REPO_ROOT/$rel"
    fi
  done < "$file"
}

remove_created_dirs() {
  local file="$BACKUP_ROOT/created_dirs.txt"
  [[ -f "$file" ]] || return 0
  awk '{ print length($0) "\t" $0 }' "$file" | sort -rn | cut -f2- | \
  while IFS= read -r rel || [[ -n "$rel" ]]; do
    [[ -n "$rel" && "$rel" != "." ]] || continue
    if [[ -d "$REPO_ROOT/$rel" ]]; then
      [[ $DRY_RUN -eq 1 ]] || rmdir "$REPO_ROOT/$rel" 2>/dev/null || true
    fi
  done
}

load_overwritten_set
restore_tree "$BACKUP_ROOT/overwritten-targets"
remove_created_files
remove_created_dirs
restore_tree "$BACKUP_ROOT/original"

MODE="DONE"
[[ $DRY_RUN -eq 1 ]] && MODE="DRY-RUN"
printf '[%s] restored snapshot=%s target=%s\n' "$MODE" "$SNAPSHOT" "${target:-unknown}"
