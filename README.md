# brave-cli

A command-line interface for the Brave Search API that retrieves search results and outputs them as JSON.

## Features

- Search the web using Brave Search API
- Output results in clean JSON format
- Handle gzip-compressed API responses
- Configurable number of results

## Installation

Make sure you have Rust and Cargo installed. Then build the project:

```bash
cargo build --release
```

The executable will be available at `target/release/brave-cli`.

## Usage

First, set your Brave Search API key as an environment variable:

```bash
export BRAVE_API_KEY=your_api_key_here
```

Then run a search:

```bash
brave-cli -q "your search query"
```

### Options

- `-q, --query <QUERY>`: Search query (required)
- `-c, --count <COUNT>`: Number of results to return (default: 10)

## Output Format

The output is a JSON object with a `results` array containing search results:

```json
{
  "results": [
    {
      "title": "Result title",
      "url": "https://example.com/result",
      "description": "Description of the result"
    },
    ...
  ]
}
```

## Getting a Brave Search API Key

To use this tool, you need a Brave Search API key. You can get one by:

1. Visiting the [Brave Search API](https://brave.com/search/api/) page
2. Creating an account or signing in
3. Following the instructions to obtain an API key

## License

This project is open source and available under the MIT License.
