const express = require('express');
const { createProxyMiddleware } = require('http-proxy-middleware');
const path = require('path');

const app = express();
const port = process.env.PORT || 8080;
const apiUrl = process.env.API_URL || 'http://localhost:8000';

// Serve static files
app.use(express.static(__dirname));

// Proxy API requests to the Rust API server
app.use('/api', createProxyMiddleware({
  target: apiUrl,
  changeOrigin: true,
}));

// Health check endpoint proxy
app.use('/', createProxyMiddleware({
  target: apiUrl,
  changeOrigin: true,
  pathFilter: '/',
}));

// Serve index.html for all other routes
app.get('*', (req, res) => {
  res.sendFile(path.join(__dirname, 'index.html'));
});

app.listen(port, () => {
  console.log(`Web UI server running at http://localhost:${port}`);
  console.log(`API requests will be proxied to ${apiUrl}`);
});
