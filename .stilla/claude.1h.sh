#!/bin/sh

# requires `$HOME/.claude/.credentials.json`
path="${HOME:-~}/.claude/.credentials.json"

_json_extract() {
    sed -n 's/.*"'"$1"'"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p; s/.*"'"$1"'"[[:space:]]*:[[:space:]]*\([^,} ]*\).*/\1/p' | sed -n '1p'
}

_json_nested_extract() { echo "$response" | sed -n "s/.*\"$1\":[^{]*{[^}]*\"$2\"[[:space:]]*:[[:space:]]*\([0-9.]*\).*/\1/p"; }

response=$(echo $(curl -s "https://api.anthropic.com/api/oauth/usage" -H "Authorization: Bearer $(_json_extract access_token < "$path")") | tr '\n' ' ')

five_hour_pct=$(_json_nested_extract five_hour percentage)
five_hour_resets=$(_json_nested_extract five_hour resets_at)
seven_day_pct=$(_json_nested_extract seven_day percentage)
seven_day_resets=$(_json_nested_extract seven_day resets_at)

echo '{"title":"Claude '"${five_hour_pct%.*}"'|'"${seven_day_pct%.*}"'","items":[{"item_type":"text","title":"'"$(date -r "${five_hour_resets:-0}" '+%Y-%m-%d %I:%M %p' 2>/dev/null)"'|'"$(date -r "${seven_day_resets:-0}" '+%Y-%m-%d %I:%M %p' 2>/dev/null)"'"}]}'
