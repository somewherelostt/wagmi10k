# 🚀 WAGMI-9000: High-Performance Echo Unit

Welcome to the WAGMI-9000 project! This is a Rust-based server designed to meet the requirements of the WAGMI-9000 challenge, focusing on speed and reliability for handling a single endpoint with different request types.

## ✨ Features

- **Single Endpoint:** Implements the required `POST /wagmi` route.
- **Request Handling:** Capable of handling both ping requests (empty body) and addition requests (`{"a": X, "b": Y}`).
- **Input Validation:** Validates addition inputs to ensure `a` and `b` are non-negative numbers and their sum is `<= 100`.
- **High Performance:** Built with Rust and the Actix-web framework for excellent concurrency handling and low latency.

## 🌐 Deployed Application

The application is deployed on Railway.app and is publicly accessible. You can interact with it at the following base URL:

**Deployed URL:** `https://wagmi10k-production.up.railway.app`

**Endpoint:** `POST /wagmi`

## 🧪 Testing the Deployed Application

You can test the deployed server using `curl` or any API testing tool like Postman.

### 1. Ping Test

Send an empty JSON body to the `/wagmi` endpoint:

```bash
curl -X POST https://wagmi10k-production.up.railway.app/wagmi -H "Content-Type: application/json" -d "{}"
```

Expected Output (timestamp will vary):

```json
{
  "message": "wagmi",
  "timestamp": "...",
  "lang": "Rust"
}
```

### 2. Addition Test

Send a JSON body with `a` and `b` to the `/wagmi` endpoint:

```bash
curl -X POST https://wagmi10k-production.up.railway.app/wagmi -H "Content-Type: application/json" -d '{"a": 40, "b": 55}'
```

Expected Output:

```json
{
  "result": 95,
  "a": 40,
  "b": 55,
  "status": "success"
}
```

### 3. Invalid Input Test

Test with invalid inputs (e.g., sum > 100 or non-numeric values):

```bash
curl -X POST https://wagmi10k-production.up.railway.app/wagmi -H "Content-Type: application/json" -d '{"a": 50, "b": 60}'
```

Expected Output:

```json
{
  "error": "Invalid input"
}
```

### 4. Load Test (10,000 Concurrent Requests)

To test the server's ability to handle high traffic, you can use the provided Node.js load testing script (`load_test.js`).

**Prerequisites:**

- Node.js and npm installed.

**Steps:**

1. Save the following code as `load_test.js` in a local directory:

    ```javascript
    const axios = require('axios');

    const TOTAL_REQUESTS = 10000;
    const CONCURRENT_REQUESTS = 1000; // Adjust for different concurrency levels
    const URL = 'https://wagmi10k-production.up.railway.app/wagmi'; // YOUR DEPLOYED RAILWAY URL

    async function makeRequest() {
        try {
            const response = await axios.post(URL, { a: Math.floor(Math.random() * 50), b: Math.floor(Math.random() * 50) }); // Using random numbers for a+b <= 100
            return response.data;
        } catch (error) {
            // console.error('Request failed:', error.message); // Uncomment for detailed errors
            return null; // Treat failed requests as null results
        }
    }

    async function runLoadTest() {
        console.log(`Starting load test with ${TOTAL_REQUESTS} total requests...`);
        const startTime = Date.now();

        const batches = Math.ceil(TOTAL_REQUESTS / CONCURRENT_REQUESTS);
        let successCount = 0;
        let failCount = 0;

        for (let i = 0; i < batches; i++) {
            const batchSize = Math.min(CONCURRENT_REQUESTS, TOTAL_REQUESTS - i * CONCURRENT_REQUESTS);
             if (batchSize <= 0) break; // Stop if no more requests

            const batchPromises = Array(batchSize)
                .fill()
                .map(() => makeRequest());

            const results = await Promise.all(batchPromises);

            results.forEach(result => {
                if (result && result.status === 'success') {
                    successCount++;
                } else {
                    failCount++;
                }
            });

            console.log(`Batch ${i + 1}/${batches} completed. Success: ${successCount}, Failed: ${failCount}`);
        }

        const endTime = Date.now();
        const duration = (endTime - startTime) / 1000;

        console.log('\nLoad Test Results:');
        console.log(`Total Duration: ${duration.toFixed(2)} seconds`);
        console.log(`Successful Requests: ${successCount}`);
        console.log(`Failed Requests: ${failCount}`);
        console.log(`Requests per second: ${(TOTAL_REQUESTS / duration).toFixed(2)}`);
    }

    runLoadTest().catch(console.error);
    ```

2. Open your terminal or command prompt, navigate to the directory where you saved `load_test.js`, and run:

    ```bash

npm init -y
npm install axios
node load_test.js

```

The script will execute the load test and print the results to the console.

## ⚙️ Local Development

If you want to run the server locally:

1.  **Install Rust:** Follow the instructions on [https://rustup.rs/](https://rustup.rs/).
2.  **Run the server:**
    ```bash
cargo run --release
```

    The server will run on `http://localhost:8080`.

## 📦 Deployment (Railway.app)

This project includes a `Dockerfile` and `railway.toml` for easy deployment on [Railway.app](https://railway.app/). Connect your GitHub repository to Railway, and it will automatically build and deploy the application.

---

*This project is part of the WAGMI-9000 challenge.*
