const axios = require("axios");

const TOTAL_REQUESTS = 10000;
const CONCURRENT_REQUESTS = 1000;
const URL = "http://localhost:8080/wagmi";

async function makeRequest() {
  try {
    const response = await axios.post(URL, { a: 40, b: 55 });
    return response.data;
  } catch (error) {
    console.error("Request failed:", error.message);
    return null;
  }
}

async function runLoadTest() {
  console.log(`Starting load test with ${TOTAL_REQUESTS} total requests...`);
  const startTime = Date.now();

  const batches = Math.ceil(TOTAL_REQUESTS / CONCURRENT_REQUESTS);
  let successCount = 0;
  let failCount = 0;

  for (let i = 0; i < batches; i++) {
    const batchPromises = Array(
      Math.min(CONCURRENT_REQUESTS, TOTAL_REQUESTS - i * CONCURRENT_REQUESTS)
    )
      .fill()
      .map(() => makeRequest());

    const results = await Promise.all(batchPromises);

    results.forEach((result) => {
      if (result && result.status === "success") {
        successCount++;
      } else {
        failCount++;
      }
    });

    console.log(
      `Batch ${
        i + 1
      }/${batches} completed. Success: ${successCount}, Failed: ${failCount}`
    );
  }

  const endTime = Date.now();
  const duration = (endTime - startTime) / 1000;

  console.log("\nLoad Test Results:");
  console.log(`Total Duration: ${duration.toFixed(2)} seconds`);
  console.log(`Successful Requests: ${successCount}`);
  console.log(`Failed Requests: ${failCount}`);
  console.log(`Requests per second: ${(TOTAL_REQUESTS / duration).toFixed(2)}`);
}

runLoadTest().catch(console.error);
