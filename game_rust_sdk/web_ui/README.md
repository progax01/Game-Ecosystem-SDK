# Game Token SDK Web UI

This is a beautiful and interactive web interface for the Game Token SDK API. It allows users to explore the API documentation and generate call data for Ethereum contract interactions.

## Features

- **Interactive API Documentation**: Comprehensive documentation with examples for all endpoints
- **API Playground**: Test all API endpoints directly from the browser
- **Token Flow Visualization**: Visual representation of token creation and burning flows
- **Responsive Design**: Works on desktop and mobile devices

## Getting Started

### Prerequisites

- Web server (for production) or you can use any local development server

### Installation

1. Clone the repository
2. Navigate to the `web_ui` directory
3. Serve the files using a web server

For development, you can use a simple HTTP server:

```bash
# Using Python
python -m http.server 8080

# Using Node.js
npx serve
```

Then open your browser and navigate to `http://localhost:8080`

## Configuration

By default, the UI is configured to connect to an API running at `http://localhost:3000`. If your API is running on a different host or port, you need to modify the `baseUrl` variable in `script.js`.

## Usage

The web UI provides several sections:

1. **Home**: Overview of the SDK capabilities
2. **Documentation**: Detailed API documentation with request/response examples
3. **API Playground**: Interactive forms to test API endpoints
4. **Token Flows**: Visual representation of token creation and burning flows

### API Playground

The API Playground allows you to:

1. Select an endpoint from the dropdown menu
2. Fill in the required parameters
3. Send a request and view the response
4. Copy the response to clipboard

### Token Flows

The Token Flows section provides a visual representation of:

1. **Create Token Flow**: CRIDA → XP → Deploy GameToken
2. **Burn Token Flow**: Burn GameToken → Get XP

## Integration with Your API

To integrate with your actual API (instead of the simulation):

1. Open `script.js`
2. Find the `sendApiRequest` function
3. Uncomment the actual fetch call and comment out the simulation code

## Customization

You can customize the UI by modifying:

- `styles.css` for visual appearance
- `index.html` for structure and content
- `script.js` for functionality

## License

This project is licensed under the MIT License - see the LICENSE file for details. 