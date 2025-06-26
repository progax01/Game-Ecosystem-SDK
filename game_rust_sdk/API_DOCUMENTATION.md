# Game Token SDK API Documentation

This API provides endpoints for generating Ethereum contract call data for Game Token operations.

> **Note**: A beautiful interactive web UI is available for exploring and testing this API. Start both the API server and web UI with `./start_servers.sh` and navigate to `http://localhost:8080` in your browser.

## Base URL

```
http://localhost:3000
```

## Authentication

This API does not use authentication.

## Common Error Responses

All endpoints may return the following error responses:

### 400 Bad Request

Returned when request validation fails or input parameters are invalid.

```json
{
  "message": "Validation error: ...",
  "status_code": 400,
  "error_type": "VALIDATION_ERROR"
}
```

### 500 Internal Server Error

Returned when an unexpected error occurs.

```json
{
  "message": "SDK error: ...",
  "status_code": 500,
  "error_type": "SDK_ERROR"
}
```

## API Endpoints

### Health Check

```
GET /
```

Response:

```json
{
  "status": "ok",
  "version": "0.1.0"
}
```

### Generate CRIDA Approval Call Data

```
POST /api/v1/calldata/creda-approve
```

Request:

```json
{
  "spender": "0x1234567890123456789012345678901234567890",
  "amount": "1000000000000000000000"
}
```

Response:

```json
{
  "calldata": "0x095ea7b3000000000000000000000000123456789012345678901234567890123456789000000000000000000000000000000000000000000000003635c9adc5dea00000"
}
```

### Generate XP Approval Call Data

```
POST /api/v1/calldata/approve-xp
```

Request:

```json
{
  "spender": "0x1234567890123456789012345678901234567890",
  "amount": "1000000000000000000000"
}
```

Response:

```json
{
  "calldata": "0x095ea7b3000000000000000000000000123456789012345678901234567890123456789000000000000000000000000000000000000000000000003635c9adc5dea00000"
}
```

### Generate Lock CREDA Call Data

```
POST /api/v1/calldata/lock-creda
```

Request:

```json
{
  "amount": "1000000000000000000000"
}
```

Response:

```json
{
  "calldata": "0xbec697db00000000000000000000000000000000000000000000003635c9adc5dea00000"
}
```

### Generate Create Game Token Call Data

```
POST /api/v1/calldata/create-token
```

Request:

```json
{
  "xp_amount": "1000000000000000000000",
  "name": "My Game Token",
  "symbol": "MGT",
  "decimals": 18
}
```

Response:

```json
{
  "calldata": "0xfd44d27400000000000000000000000000000000000000000000003635c9adc5dea00000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000012000000000000000000000000000000000000000000000000000000000000000d4d792047616d6520546f6b656e000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000034d4754000000000000000000000000000000000000000000000000000000000"
}
```

### Generate Burn Game Token Call Data

```
POST /api/v1/calldata/burn-token
```

Request:

```json
{
  "game_id": "1",
  "amount": "500000000000000000000"
}
```

Response:

```json
{
  "calldata": "0x5ed6c1db0000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000001b1ae4d6e2ef500000"
}
```

### Generate Complete Create Token Flow Call Data

```
POST /api/v1/flow/create
```

Request:

```json
{
  "factory_address": "0x1234567890123456789012345678901234567890",
  "creda_amount": "1000000000000000000000",
  "game_name": "My Game Token",
  "game_symbol": "MGT",
  "decimals": 18
}
```

Response:

```json
{
  "creda_approve": "0x095ea7b3000000000000000000000000123456789012345678901234567890123456789000000000000000000000000000000000000000000000003635c9adc5dea00000",
  "lock_creda": "0xbec697db00000000000000000000000000000000000000000000003635c9adc5dea00000",
  "xp_amount": "1000000000000000000000",
  "xp_approve": "0x095ea7b3000000000000000000000000123456789012345678901234567890123456789000000000000000000000000000000000000000000000003635c9adc5dea00000",
  "create_token": "0xfd44d27400000000000000000000000000000000000000000000003635c9adc5dea00000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000012000000000000000000000000000000000000000000000000000000000000000d4d792047616d6520546f6b656e000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000034d4754000000000000000000000000000000000000000000000000000000000"
}
```

### Generate Burn Token Flow Call Data

```
POST /api/v1/flow/burn
```

Request:

```json
{
  "factory_address": "0x1234567890123456789012345678901234567890",
  "game_token_address": "0xabcdef0123456789abcdef0123456789abcdef01",
  "game_id": "1",
  "burn_amount": "500000000000000000000"
}
```

Response:

```json
{
  "game_token_approve": "0x095ea7b3000000000000000000000000123456789012345678901234567890123456789000000000000000000000000000000000000000000000001b1ae4d6e2ef500000",
  "burn_game_token": "0x5ed6c1db0000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000001b1ae4d6e2ef500000"
}
```

## Using with JavaScript Clients

Here's an example of how to use this API with a JavaScript client:

```javascript
// Example: Call the API to get the complete flow for creating a game token
async function getCreateTokenFlow() {
  const response = await fetch('http://localhost:3000/api/v1/flow/create', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      factory_address: '0x1234567890123456789012345678901234567890',
      creda_amount: '1000000000000000000000',  // 1000 tokens with 18 decimals
      game_name: 'My Game Token',
      game_symbol: 'MGT',
      decimals: 18,
    }),
  });

  if (!response.ok) {
    const error = await response.json();
    throw new Error(`API Error: ${error.message}`);
  }

  const data = await response.json();
  return data;
}

// Example: Using the API to send a transaction with ethers.js
async function createGameToken() {
  const flow = await getCreateTokenFlow();
  
  // Step 1: Approve CREDA tokens
  const provider = new ethers.providers.Web3Provider(window.ethereum);
  await window.ethereum.request({ method: 'eth_requestAccounts' });
  const signer = provider.getSigner();
  
  // These addresses would come from your configuration
  const credaTokenAddress = '0xYourCredaTokenAddress';
  const factoryAddress = '0x1234567890123456789012345678901234567890';
  
  // Send the CRIDA approval transaction
  const tx1 = {
    to: credaTokenAddress,
    data: flow.creda_approve,
  };
  await signer.sendTransaction(tx1);
  
  // Continue with other steps in the flow...
}
```

## Running the API Server

To run the API server:

```bash
# Set the port (optional, default is 3000)
export PORT=8080

# Run the server
cargo run --bin api_server
```

Or using the release build for better performance:

```bash
cargo run --release --bin api_server
``` 