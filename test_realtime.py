#!/usr/bin/env python3
"""
Test script for ResearchProcess-GPS real-time WebSocket functionality.
Tests that events flow from REST API operations to WebSocket clients.
"""

import asyncio
import json
import sys
import uuid
from datetime import datetime
import websockets
import aiohttp

BASE_URL = "http://localhost:8080"
WS_URL = "ws://localhost:8080/api/v1/ws"

async def test_realtime():
    print("Testing ResearchProcess-GPS Real-time Events")
    print("=" * 50)
    
    # Track received events
    received_events = []
    
    # Connect to WebSocket
    print("\n1. Connecting to WebSocket...")
    async with websockets.connect(WS_URL) as websocket:
        print("   ✓ Connected to WebSocket")
        
        # Set up message handler
        async def handle_messages():
            try:
                async for message in websocket:
                    data = json.loads(message)
                    print(f"\n   📨 Received: {json.dumps(data, indent=2)}")
                    
                    # Track events
                    if data.get("type") == "event":
                        received_events.append(data)
            except websockets.exceptions.ConnectionClosed:
                pass
        
        # Start message handler
        message_task = asyncio.create_task(handle_messages())
        
        # Test authentication
        print("\n2. Authenticating...")
        auth_msg = {
            "id": "auth-1",
            "type": "auth",
            "token": "test-token"
        }
        await websocket.send(json.dumps(auth_msg))
        await asyncio.sleep(0.5)  # Wait for auth response
        
        # Create a test entity ID
        test_entity_id = str(uuid.uuid4())
        
        # Subscribe to entity updates
        print(f"\n3. Subscribing to entity {test_entity_id}...")
        sub_msg = {
            "id": "sub-1",
            "type": "subscribe",
            "subscription_type": "entity",
            "params": {
                "entity_id": test_entity_id
            }
        }
        await websocket.send(json.dumps(sub_msg))
        await asyncio.sleep(0.5)  # Wait for subscription confirmation
        
        # Now create the entity via REST API
        print(f"\n4. Creating entity via REST API...")
        async with aiohttp.ClientSession() as session:
            # Create entity
            create_data = {
                "entity_type": "Theory",
                "data": {
                    "question": "Test real-time events",
                    "hypothesis": "WebSocket will receive event notifications",
                    "status": "EXPLORING"
                }
            }
            
            # Override the ID in the URL
            async with session.post(
                f"{BASE_URL}/api/v1/entities",
                json=create_data,
                headers={"X-Entity-ID": test_entity_id}  # If supported
            ) as resp:
                if resp.status == 201:
                    entity = await resp.json()
                    created_id = entity["id"]
                    print(f"   ✓ Created entity: {created_id}")
                else:
                    print(f"   ✗ Failed to create entity: {resp.status}")
                    return
            
            # Wait for event
            await asyncio.sleep(1)
            
            # Update entity
            print(f"\n5. Updating entity via REST API...")
            update_data = {
                "entity_type": "Theory",
                "data": {
                    "status": "TESTING",
                    "conclusion": "WebSocket events are working!"
                }
            }
            
            async with session.put(
                f"{BASE_URL}/api/v1/entities/{created_id}",
                json=update_data
            ) as resp:
                if resp.status == 200:
                    print(f"   ✓ Updated entity")
                else:
                    print(f"   ✗ Failed to update entity: {resp.status}")
            
            # Wait for event
            await asyncio.sleep(1)
            
            # Delete entity
            print(f"\n6. Deleting entity via REST API...")
            async with session.delete(
                f"{BASE_URL}/api/v1/entities/{created_id}"
            ) as resp:
                if resp.status == 204:
                    print(f"   ✓ Deleted entity")
                else:
                    print(f"   ✗ Failed to delete entity: {resp.status}")
            
            # Wait for event
            await asyncio.sleep(1)
        
        # Cancel message handler
        message_task.cancel()
        
        # Summary
        print(f"\n7. Summary:")
        print(f"   Total events received: {len(received_events)}")
        if len(received_events) > 0:
            print("   ✅ Real-time events are working!")
        else:
            print("   ❌ No events received - check LISTEN/NOTIFY setup")
    
    print("\n✓ Test completed")

if __name__ == "__main__":
    asyncio.run(test_realtime())