#!/usr/bin/env bash
set -euo pipefail
# Enkla röktester mot lokalt körande mock
curl -s -H "Client-Secret: dev-secret" http://localhost:4000/api/v1/customers | jq .
curl -s -H "Client-Secret: dev-secret" http://localhost:4000/api/v1/purchase-orders | jq .
