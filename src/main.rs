use clap::{Arg, Command};
use flate2::read::GzDecoder;
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::io::Read;

// Define structures for Brave Search API response
#[derive(Debug, Deserialize)]
struct BraveSearchResponse {
    #[serde(default)]
    web: Web,
}

#[derive(Debug, Deserialize, Default)]
struct Web {
    #[serde(default)]
    results: Vec<WebResult>,
}

#[derive(Debug, Deserialize)]
struct WebResult {
    title: String,
    url: String,
    description: String,
}

// Structure for saving collected URLs
#[derive(Debug, Serialize)]
struct CollectedUrl {
    title: String,
    url: String,
    description: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("Brave Search CLI")
        .version("0.1.0")
        .author("Shane Witbeck <shane@surly.dev>")
        .about("Search the web using the Brave Search API")
        .arg(
            Arg::new("query")
                .short('q')
                .long("query")
                .value_name("QUERY")
                .help("The query to search for")
                .required(true),
        )
        // API key is now retrieved from environment variable
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .value_name("COUNT")
                .help("Number of results to retrieve (default: 10)")
                .default_value("10"),
        )
        // Output is now printed to stdout
        .get_matches();

    let query = matches.get_one::<String>("query").unwrap();
    let api_key = env::var("BRAVE_API_KEY").expect("BRAVE_API_KEY environment variable not set");
    let count = matches.get_one::<String>("count").unwrap().parse::<u32>().unwrap_or(10);

    // Create HTTP client with headers
    let mut headers = header::HeaderMap::new();
    headers.insert(header::ACCEPT, header::HeaderValue::from_static("application/json"));
    headers.insert(header::ACCEPT_ENCODING, header::HeaderValue::from_static("gzip"));
    headers.insert(
        "X-Subscription-Token",
        header::HeaderValue::from_str(&api_key)?
    );
    
    // In reqwest 0.12, gzip compression is enabled by default
    let http_client = Client::builder()
        .default_headers(headers)
        .build()?;

    // Make request to Brave Search API
    let url = format!(
        "https://api.search.brave.com/res/v1/web/search?q={}&count={}",
        urlencoding::encode(query),
        count
    );
    
    let response = http_client.get(&url).send().await?;
    
    if !response.status().is_success() {
        return Err(format!("API request failed with status: {}", response.status()).into());
    }
    
    // Get the response bytes
    let response_bytes = response.bytes().await?;
    
    if response_bytes.is_empty() {
        return Err("Empty response from API".into());
    }
    
    // Check if the response is gzip-compressed (gzip magic number: 1f 8b)
    let is_gzip = response_bytes.len() >= 2 && response_bytes[0] == 0x1f && response_bytes[1] == 0x8b;
    
    let search_response: BraveSearchResponse = if is_gzip {
        // Decompress the gzip data
        let mut decoder = GzDecoder::new(&response_bytes[..]);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)
            .map_err(|e| format!("Failed to decompress gzip data: {}", e))?;
        
        // Parse the JSON from the decompressed data
        serde_json::from_slice(&decompressed)
            .map_err(|e| format!("Failed to parse decompressed JSON: {}", e))?
    } else {
        // Try to parse the response directly from bytes
        serde_json::from_slice(&response_bytes)
            .map_err(|e| format!("Failed to parse API response: {}", e))?
    };
    
    // Extract and collect URLs
    let collected_urls: Vec<CollectedUrl> = search_response.web.results
        .into_iter()
        .map(|result| CollectedUrl {
            title: result.title,
            url: result.url,
            description: result.description,
        })
        .collect();
    
    // Wrap the collected URLs in a results object
    let results = serde_json::json!({
        "results": collected_urls
    });
    
    // Print the results as JSON to stdout
    let json = serde_json::to_string_pretty(&results)?;
    println!("{}", json);
    
    Ok(())
}
