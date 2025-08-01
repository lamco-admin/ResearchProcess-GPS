#!/bin/bash
# Test script for WebSocket functionality

echo "Testing ResearchProcess-GPS WebSocket endpoint..."
echo ""

# Check if wscat is installed
if ! command -v wscat &> /dev/null; then
    echo "wscat is not installed. Please install it with: npm install -g wscat"
    exit 1
fi

# WebSocket URL
WS_URL="ws://localhost:8080/api/v1/ws"

echo "1. Connecting to WebSocket at $WS_URL"
echo "   Commands to test:"
echo "   - Authentication: {\"id\":\"auth-1\",\"type\":\"auth\",\"token\":\"test-token\"}"
echo "   - Subscribe to entity: {\"id\":\"sub-1\",\"type\":\"subscribe\",\"subscription_type\":\"entity\",\"params\":{\"entity_id\":\"019861ac-4581-7393-b484-7a8a26273217\"}}"
echo "   - Unsubscribe: {\"id\":\"unsub-1\",\"type\":\"unsubscribe\",\"subscription_id\":\"<subscription_id>\"}"
echo ""
echo "Connecting..."

# Connect to WebSocket
wscat -c "$WS_URL"