document.addEventListener('DOMContentLoaded', function() {
    // Initialize code highlighting
    hljs.highlightAll();
    
    // Setup navigation
    setupNavigation();
    
    // Setup API playground
    setupApiPlayground();
    
    // Setup copy button
    setupCopyButton();
});

// Navigation between sections
function setupNavigation() {
    const navLinks = document.querySelectorAll('.navbar-nav .nav-link');
    
    navLinks.forEach(link => {
        link.addEventListener('click', function(e) {
            e.preventDefault();
            
            // Remove active class from all links
            navLinks.forEach(l => l.classList.remove('active'));
            
            // Add active class to clicked link
            this.classList.add('active');
            
            // Show the corresponding section
            const sectionId = this.getAttribute('data-section');
            navigateTo(sectionId);
        });
    });
}

// Show a specific section
function navigateTo(sectionId) {
    // Hide all sections
    document.querySelectorAll('.section').forEach(section => {
        section.classList.remove('active');
    });
    
    // Show the selected section
    document.getElementById(sectionId).classList.add('active');
    
    // Update navigation
    document.querySelectorAll('.navbar-nav .nav-link').forEach(link => {
        if (link.getAttribute('data-section') === sectionId) {
            link.classList.add('active');
        } else {
            link.classList.remove('active');
        }
    });
    
    // Collapse navbar on mobile
    const navbarCollapse = document.querySelector('.navbar-collapse');
    if (navbarCollapse.classList.contains('show')) {
        navbarCollapse.classList.remove('show');
    }
    
    // Scroll to top
    window.scrollTo(0, 0);
}

// API Playground functionality
function setupApiPlayground() {
    const endpointSelect = document.getElementById('endpoint');
    const formContainer = document.getElementById('form-container');
    const sendRequestBtn = document.getElementById('send-request');
    const responseContainer = document.getElementById('response-container');
    
    // Initial form setup
    updateFormFields(endpointSelect.value);
    
    // Update form when endpoint changes
    endpointSelect.addEventListener('change', function() {
        updateFormFields(this.value);
    });
    
    // Send request button click
    sendRequestBtn.addEventListener('click', function() {
        sendApiRequest();
    });
}

// Update form fields based on selected endpoint
function updateFormFields(endpoint) {
    const formContainer = document.getElementById('form-container');
    formContainer.innerHTML = '';
    
    switch (endpoint) {
        case 'health':
            // No fields needed for health check
            break;
            
        case 'creda-approve':
        case 'approve-xp':
            formContainer.innerHTML = `
                <div class="mb-3">
                    <label for="spender" class="form-label">Spender Address</label>
                    <input type="text" class="form-control" id="spender" placeholder="0x1234...">
                    <div class="form-text">Ethereum address of the spender</div>
                </div>
                <div class="mb-3">
                    <label for="amount" class="form-label">Amount</label>
                    <input type="text" class="form-control" id="amount" placeholder="1000000000000000000000">
                    <div class="form-text">Amount in wei (1 ETH = 10^18 wei)</div>
                </div>
            `;
            break;
            
        case 'lock-creda':
            formContainer.innerHTML = `
                <div class="mb-3">
                    <label for="amount" class="form-label">Amount</label>
                    <input type="text" class="form-control" id="amount" placeholder="1000000000000000000000">
                    <div class="form-text">Amount of CRIDA tokens to lock (in wei)</div>
                </div>
            `;
            break;
            
        case 'create-token':
            formContainer.innerHTML = `
                <div class="mb-3">
                    <label for="xp_amount" class="form-label">XP Amount</label>
                    <input type="text" class="form-control" id="xp_amount" placeholder="1000000000000000000000">
                    <div class="form-text">Amount of XP tokens to use (in wei)</div>
                </div>
                <div class="mb-3">
                    <label for="name" class="form-label">Token Name</label>
                    <input type="text" class="form-control" id="name" placeholder="My Game Token">
                </div>
                <div class="mb-3">
                    <label for="symbol" class="form-label">Token Symbol</label>
                    <input type="text" class="form-control" id="symbol" placeholder="MGT">
                </div>
                <div class="mb-3">
                    <label for="decimals" class="form-label">Decimals</label>
                    <input type="number" class="form-control" id="decimals" placeholder="18" min="0" max="18">
                    <div class="form-text">Number of decimal places (0-18)</div>
                </div>
            `;
            break;
            
        case 'burn-token':
            formContainer.innerHTML = `
                <div class="mb-3">
                    <label for="game_id" class="form-label">Game ID</label>
                    <input type="text" class="form-control" id="game_id" placeholder="1">
                </div>
                <div class="mb-3">
                    <label for="amount" class="form-label">Amount</label>
                    <input type="text" class="form-control" id="amount" placeholder="500000000000000000000">
                    <div class="form-text">Amount of game tokens to burn (in wei)</div>
                </div>
            `;
            break;
            
        case 'flow-create':
            formContainer.innerHTML = `
                <div class="mb-3">
                    <label for="factory_address" class="form-label">Factory Address</label>
                    <input type="text" class="form-control" id="factory_address" placeholder="0x1234...">
                    <div class="form-text">Ethereum address of the GameTokenFactory contract</div>
                </div>
                <div class="mb-3">
                    <label for="creda_amount" class="form-label">CRIDA Amount</label>
                    <input type="text" class="form-control" id="creda_amount" placeholder="1000000000000000000000">
                    <div class="form-text">Amount of CRIDA tokens to lock (in wei)</div>
                </div>
                <div class="mb-3">
                    <label for="game_name" class="form-label">Game Token Name</label>
                    <input type="text" class="form-control" id="game_name" placeholder="My Game Token">
                </div>
                <div class="mb-3">
                    <label for="game_symbol" class="form-label">Game Token Symbol</label>
                    <input type="text" class="form-control" id="game_symbol" placeholder="MGT">
                </div>
                <div class="mb-3">
                    <label for="decimals" class="form-label">Decimals</label>
                    <input type="number" class="form-control" id="decimals" placeholder="18" min="0" max="18">
                    <div class="form-text">Number of decimal places (0-18)</div>
                </div>
            `;
            break;
            
        case 'flow-burn':
            formContainer.innerHTML = `
                <div class="mb-3">
                    <label for="factory_address" class="form-label">Factory Address</label>
                    <input type="text" class="form-control" id="factory_address" placeholder="0x1234...">
                    <div class="form-text">Ethereum address of the GameTokenFactory contract</div>
                </div>
                <div class="mb-3">
                    <label for="game_token_address" class="form-label">Game Token Address</label>
                    <input type="text" class="form-control" id="game_token_address" placeholder="0xabcd...">
                    <div class="form-text">Ethereum address of the game token contract</div>
                </div>
                <div class="mb-3">
                    <label for="game_id" class="form-label">Game ID</label>
                    <input type="text" class="form-control" id="game_id" placeholder="1">
                </div>
                <div class="mb-3">
                    <label for="burn_amount" class="form-label">Burn Amount</label>
                    <input type="text" class="form-control" id="burn_amount" placeholder="500000000000000000000">
                    <div class="form-text">Amount of game tokens to burn (in wei)</div>
                </div>
            `;
            break;
    }
}

// Send API request
async function sendApiRequest() {
    const endpoint = document.getElementById('endpoint').value;
    const responseContainer = document.getElementById('response-container');
    
    // Show loading state
    responseContainer.textContent = 'Loading...';
    
    try {
        let url = '';
        let method = 'GET';
        let body = null;
        
        // Prepare request based on endpoint
        switch (endpoint) {
            case 'health':
                url = '/';
                break;
                
            case 'creda-approve':
                url = '/api/v1/calldata/creda-approve';
                method = 'POST';
                body = {
                    spender: document.getElementById('spender').value,
                    amount: document.getElementById('amount').value
                };
                break;
                
            case 'approve-xp':
                url = '/api/v1/calldata/approve-xp';
                method = 'POST';
                body = {
                    spender: document.getElementById('spender').value,
                    amount: document.getElementById('amount').value
                };
                break;
                
            case 'lock-creda':
                url = '/api/v1/calldata/lock-creda';
                method = 'POST';
                body = {
                    amount: document.getElementById('amount').value
                };
                break;
                
            case 'create-token':
                url = '/api/v1/calldata/create-token';
                method = 'POST';
                body = {
                    xp_amount: document.getElementById('xp_amount').value,
                    name: document.getElementById('name').value,
                    symbol: document.getElementById('symbol').value,
                    decimals: parseInt(document.getElementById('decimals').value)
                };
                break;
                
            case 'burn-token':
                url = '/api/v1/calldata/burn-token';
                method = 'POST';
                body = {
                    game_id: document.getElementById('game_id').value,
                    amount: document.getElementById('amount').value
                };
                break;
                
            case 'flow-create':
                url = '/api/v1/flow/create';
                method = 'POST';
                body = {
                    factory_address: document.getElementById('factory_address').value,
                    creda_amount: document.getElementById('creda_amount').value,
                    game_name: document.getElementById('game_name').value,
                    game_symbol: document.getElementById('game_symbol').value,
                    decimals: parseInt(document.getElementById('decimals').value)
                };
                break;
                
            case 'flow-burn':
                url = '/api/v1/flow/burn';
                method = 'POST';
                body = {
                    factory_address: document.getElementById('factory_address').value,
                    game_token_address: document.getElementById('game_token_address').value,
                    game_id: document.getElementById('game_id').value,
                    burn_amount: document.getElementById('burn_amount').value
                };
                break;
        }
        
        // For demonstration purposes, we'll simulate the API response
        // In a real application, you would make an actual fetch call to your API
        const baseUrl = 'http://localhost:3000';
        
        // Simulate API call
        // In production, use this:
        // const response = await fetch(baseUrl + url, {
        //     method: method,
        //     headers: {
        //         'Content-Type': 'application/json'
        //     },
        //     body: method === 'POST' ? JSON.stringify(body) : undefined
        // });
        // const data = await response.json();
        
        // Simulate API response for demo
        let data = simulateApiResponse(endpoint, body);
        
        // Display response
        responseContainer.textContent = JSON.stringify(data, null, 2);
        hljs.highlightElement(responseContainer);
        
    } catch (error) {
        responseContainer.textContent = JSON.stringify({
            error: true,
            message: error.message
        }, null, 2);
        hljs.highlightElement(responseContainer);
    }
}

// Simulate API response for demonstration
function simulateApiResponse(endpoint, body) {
    switch (endpoint) {
        case 'health':
            return {
                status: 'ok',
                version: '0.1.0'
            };
            
        case 'creda-approve':
        case 'approve-xp':
            return {
                calldata: `0x095ea7b3000000000000000000000000${body.spender.substring(2)}00000000000000000000000000000000000000000000003635c9adc5dea00000`
            };
            
        case 'lock-creda':
            return {
                calldata: `0xbec697db00000000000000000000000000000000000000000000003635c9adc5dea00000`
            };
            
        case 'create-token':
            return {
                calldata: `0xfd44d27400000000000000000000000000000000000000000000003635c9adc5dea00000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000012000000000000000000000000000000000000000000000000000000000000000d${stringToHex(body.name)}000000000000000000000000000000000000000000000000000000000000000${stringToHex(body.symbol)}000000000000000000000000000000000000000000000000000000`
            };
            
        case 'burn-token':
            return {
                calldata: `0x5ed6c1db000000000000000000000000000000000000000000000000000000000000000${body.game_id}000000000000000000000000000000000000000000000001b1ae4d6e2ef500000`
            };
            
        case 'flow-create':
            return {
                creda_approve: `0x095ea7b3000000000000000000000000${body.factory_address.substring(2)}00000000000000000000000000000000000000000000003635c9adc5dea00000`,
                lock_creda: `0xbec697db00000000000000000000000000000000000000000000003635c9adc5dea00000`,
                xp_amount: body.creda_amount,
                xp_approve: `0x095ea7b3000000000000000000000000${body.factory_address.substring(2)}00000000000000000000000000000000000000000000003635c9adc5dea00000`,
                create_token: `0xfd44d27400000000000000000000000000000000000000000000003635c9adc5dea00000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000012000000000000000000000000000000000000000000000000000000000000000d${stringToHex(body.game_name)}000000000000000000000000000000000000000000000000000000000000000${stringToHex(body.game_symbol)}000000000000000000000000000000000000000000000000000000`
            };
            
        case 'flow-burn':
            return {
                game_token_approve: `0x095ea7b3000000000000000000000000${body.factory_address.substring(2)}000000000000000000000000000000000000000000000001b1ae4d6e2ef500000`,
                burn_game_token: `0x5ed6c1db000000000000000000000000000000000000000000000000000000000000000${body.game_id}000000000000000000000000000000000000000000000001b1ae4d6e2ef500000`
            };
            
        default:
            return {
                error: 'Unknown endpoint'
            };
    }
}

// Helper function to convert string to hex
function stringToHex(str) {
    let hex = '';
    for (let i = 0; i < str.length; i++) {
        hex += str.charCodeAt(i).toString(16);
    }
    return hex;
}

// Setup copy button
function setupCopyButton() {
    const copyBtn = document.getElementById('copy-response');
    const responseContainer = document.getElementById('response-container');
    
    copyBtn.addEventListener('click', function() {
        const text = responseContainer.textContent;
        navigator.clipboard.writeText(text).then(function() {
            // Change button text temporarily
            const originalText = copyBtn.innerHTML;
            copyBtn.innerHTML = '<i class="bi bi-check-circle me-2"></i>Copied!';
            
            // Reset button text after 2 seconds
            setTimeout(function() {
                copyBtn.innerHTML = originalText;
            }, 2000);
        });
    });
}

// Set endpoint in the playground
function setEndpoint(endpoint) {
    const endpointSelect = document.getElementById('endpoint');
    endpointSelect.value = endpoint;
    updateFormFields(endpoint);
} 