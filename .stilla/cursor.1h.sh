#!/bin/sh

# requires [WorkosCursorSessionToken](https://cursor.com/api/usage-summary) (request headers -> cookies -> WorkosCursorSessionToken)
WorkosCursorSessionToken=""

_json_extract() {
  sed -n "s/.*\"$1\"[[:space:]]*:[[:space:]]*\([0-9][0-9]*\).*/\1/p"
}

response=$(echo $(curl -s "https://cursor.com/api/usage-summary" -b "WorkosCursorSessionToken=$WorkosCursorSessionToken") | tr '\n' ' ')

used=$(echo "$response" | sed -n 's/.*"overall":{[^}]*"used":[[:space:]]*\([0-9][0-9]*\).*/\1/p')
limit=$(echo "$response" | sed -n 's/.*"limit":[[:space:]]*\([0-9][0-9]*\).*/\1/p')

echo '{"title":"Cursor '"$(( used * 100 / limit ))"'%","items":[{"item_type":"text","title":"'"$used"' / '"$limit"'"}]}'
