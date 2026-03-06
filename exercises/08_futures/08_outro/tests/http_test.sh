#!/bin/bash
set -e

BASE="http://localhost:3000"

echo "=== POST /tickets ==="
ID=$(curl -s -X POST "$BASE/tickets" \
  -H "Content-Type: application/json" \
  -d '{"title": "Fix login bug", "description": "The login page crashes on submit"}')
echo "$ID" | jq .

echo ""
echo "=== GET /tickets/0 ==="
curl -s "$BASE/tickets/0" | jq .

echo ""
echo "=== PATCH /tickets/0 ==="
curl -s -o /dev/null -w "Status: %{http_code}\n" -X PATCH "$BASE/tickets/0" \
  -H "Content-Type: application/json" \
  -d '{"status": "InProgress"}'

echo ""
echo "=== GET /tickets/0 (after patch) ==="
curl -s "$BASE/tickets/0" | jq .

echo ""
echo "=== GET /tickets/999 (should 404) ==="
curl -s -o /dev/null -w "Status: %{http_code}\n" "$BASE/tickets/999"

