const WebSocket = require('ws');

const ws = new WebSocket('ws://localhost:8080/api/v1/ws');

console.log('Connecting to WebSocket...');

ws.on('open', function open() {
    console.log('Connected! Testing WebSocket protocol...\n');
    
    // Test 1: Try to subscribe without auth (should fail)
    console.log('1. Testing subscription without auth...');
    ws.send(JSON.stringify({
        id: 'test-1',
        type: 'subscribe',
        subscription_type: 'entity',
        params: {
            entity_id: '019861ac-4581-7393-b484-7a8a26273217'
        }
    }));
    
    // Test 2: Authenticate after a delay
    setTimeout(() => {
        console.log('\n2. Sending auth message...');
        ws.send(JSON.stringify({
            id: 'auth-1',
            type: 'auth',
            token: 'test-token'
        }));
    }, 1000);
    
    // Test 3: Subscribe after auth
    setTimeout(() => {
        console.log('\n3. Subscribing to entity...');
        ws.send(JSON.stringify({
            id: 'sub-1',
            type: 'subscribe',
            subscription_type: 'entity',
            params: {
                entity_id: '019861ac-4581-7393-b484-7a8a26273217'
            }
        }));
    }, 2000);
    
    // Close after tests
    setTimeout(() => {
        console.log('\n4. Closing connection...');
        ws.close();
    }, 4000);
});

ws.on('message', function message(data) {
    console.log('Received:', JSON.stringify(JSON.parse(data.toString()), null, 2));
});

ws.on('error', function error(err) {
    console.error('WebSocket error:', err);
});

ws.on('close', function close() {
    console.log('\nConnection closed');
});