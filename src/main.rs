use reqwest::blocking::Client;
use serde::{Serialize, Deserialize};
use std::net::{TcpStream, Shutdown};
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

// Struct to represent the data to be sent to the API
#[derive(Serialize, Deserialize)]
struct TagData {
    tag: String,
    value: f32,
}

fn main() {
    // Access environment variables
    let api_url = "http://localhost:8080/api/metrics";

    // Define micrologix address
    let micrologix_address = "192.168.1.100:44818";

    // Connect to the MicroLogix controller
    let mut stream = TcpStream::connect(micrologix_address).expect("Failed to connect to controller");

    // Define the tag name
    let tag_name = "F8:31";

    // Create a Reqwest client
    let client = Client::new();

    // Start the data collection loop
    loop {
        // Create the request message
        let request = format!("READ_TAG {}\r\n", tag_name);

        // Send the request to the controller
        stream.write(request.as_bytes()).expect("Failed to send request");

        // Read the response from the controller
        let mut response = String::new();
        stream.read_to_string(&mut response).expect("Failed to read response");

        // Parse the response value
        let value: f32 = response.trim().parse().unwrap();

        // Close the connection
        stream.shutdown(Shutdown::Both).expect("Failed to close connection");

        // Create the TagData object
        let tag_data = TagData {
            tag: tag_name.to_string(),
            value,
        };

        // Send the data to the API
        send_data_to_api(&client, api_url, &tag_data);

        // Delay for 1 second before the next data collection
        thread::sleep(Duration::from_secs(1));
    }
}

fn send_data_to_api(client: &Client, url: &str, data: &TagData) {
    // Send the data as JSON to the API
    let response = client.post(url)
        .json(data)
        .send()
        .expect("Failed to send data to API");

    // Check the response status
    if response.status().is_success() {
        println!("Data sent successfully!");
    } else {
        println!("Failed to send data to API: {}", response.status());
    }
}