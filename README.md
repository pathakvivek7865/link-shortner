<!-- Link Shortner Api -->

## Introduction

The Link Shortner API is a powerful tool for creating shortened URLs. It allows users to generate short links that redirect to longer URLs. This documentation will guide you through the setup and usage of the API.

## Setup and Run

To set up and run the Link Shortner API, follow these steps:

1. Clone the repository to your local machine:

   ```shell
   git clone https://github.com/pathakvivek7865/link-shortner.git
   ```

2. Navigate to the project directory:

   ```shell
   cd link-shortner
   ```

3. Build the project:

   ```shell
   cargo build
   ```

4. Create a database named `postgres` in your PostgreSQL server.
5. Create a `.env` file in the root directory and configure the following environment variables:
   ```shell
   DATABASE_URL=postgresql://postgres:postgres@localhost:5432/postgres
   ```

6. Start the API server:
   ```shell
   cargo run
   ```

## Routes

The Link Shortner API provides the following routes:

- `POST /create`: Creates a shortened URL. Requires a JSON payload with the `targetUrl` property.

    - Request JSON:

        ```json
        {
            "targetUrl": "https://example.com/very-long-url"
        }
        ```

    - Request Headers:

        ```
        Content-Type: application/json,
        x-api-key: <API KEY>
        ```

    - Response:
        ```json
        {
            "id": "LKKJLlaskdjf,
            "targetUrl": "https://example.com/very-long-url"
        }
        ```

- `GET /:id`: Redirects to the original URL associated with the shortened URL.
- `GET /:id/statistics`: Returns the statistics of the shortened URL.

    - Request Headers:

        ```
        Content-Type: application/json,
        x-api-key: <API KEY>
        ```

    - Response:
        ```json
        [
            {
            "amount": 8,
            "referer": "Postman",
            "userAgent": "PostmanRuntime/7.39.0"
            }
        ]
        ```
- `PATCH /:id`: Updates the target URL of the shortened URL. Requires a JSON payload with the `targetUrl` property.

    - Request JSON:

        ```json
        {
            "targetUrl": "https://example.com/new-url"
        }
        ```

    - Request Headers:

        ```
        Content-Type: application/json,
        x-api-key: <API KEY>
        ```

    - Response:
        ```json
        {
            "id": "LKKJLlaskdjf,
            "targetUrl": "https://example.com/new-url"
        }
        ```
