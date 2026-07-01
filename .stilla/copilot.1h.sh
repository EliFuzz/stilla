#!/bin/sh

# requires [GITHUB_TOKEN](https://github.com/settings/personal-access-tokens) with `Profile` scope
GITHUB_TOKEN=""

_json_extract() {
    sed -n 's/.*"'"$1"'"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p; s/.*"'"$1"'"[[:space:]]*:[[:space:]]*\([^,} ]*\).*/\1/p' | sed -n '1p'
}

response=$(echo $(curl -s "https://api.github.com/copilot_internal/user" -H "Authorization: token ${GITHUB_TOKEN}") | tr '\n' ' ')

used=$(printf '%s\n' "$response" | _json_extract "remaining")
limit=$(printf '%s\n' "$response" | _json_extract "entitlement")

echo '{"title":"Copilot '"$(( used * 100 / limit ))"'%","items":[{"item_type":"text","title":"'"$used"' / '"$limit"'"}]}'
