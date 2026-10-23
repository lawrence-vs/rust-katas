# source this file, then: kata 07
kata() {
  local root dir name
  root=$(git rev-parse --show-toplevel) || return
  local -a hits
  hits=("$root"/katas/*/"$(printf %02d "$((10#$1))")"-*(N))
  dir=${hits[1]}
  [ -n "$dir" ] || { echo "no kata $1 on this branch; merge its PR and pull main" >&2; return 1; }
  name=$(basename "$dir" | sed -E 's/^([0-9]+)-(beginner|intermediate|advanced|expert)-/\1-/')
  git switch "solve/$name" 2>/dev/null || git switch -c "solve/$name" || return
  cd "$dir" && cargo watch -q -c -x check -x test
}
