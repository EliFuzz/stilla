#!/bin/sh

path="${HOME:-~}/.codex/auth.json"
[ -f "$path" ] || exit 1

_json_extract() {
    sed -n 's/.*"'"$1"'"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p; s/.*"'"$1"'"[[:space:]]*:[[:space:]]*\([^,} ]*\).*/\1/p' | sed -n '1p'
}

response=$(echo $(curl -s "https://chatgpt.com/backend-api/wham/usage" -H "Authorization: Bearer $(_json_extract access_token < "$path")" -H "Accept: application/json") | tr '\n' ' ')

echo '{"title":"Codex '"$(printf '%s\n' "$response" | _json_extract used_percent | sed 's/\..*//')"'%","items":[{"item_type":"text","title":"'"$(printf '%s\n' "$response" | _json_extract used | sed 's/\..*//')"' / '"$(printf '%s\n' "$response" | _json_extract limit | sed 's/\..*//')"' ('"$(date -r "$(printf '%s\n' "$response" | _json_extract reset_at)" '+%Y-%m-%d %I:%M %p' 2>/dev/null)"')"}]}'
