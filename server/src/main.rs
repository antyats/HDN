use serde::{Deserialize, Serialize};
use serde_json::{Value};
use std::collections::HashMap;
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

#[derive(Serialize, Deserialize)]
struct LoadResponse {
    /// response_status: Response status to client
    /// requested_key: Requested key from client
    /// requested_hash: Requested hash from client
    response_status: String,
    requested_key: String,
    requested_hash: String,
}
///
#[derive(Serialize, Deserialize)]
struct SuccessOrFailResponse {
    response_status: String,
}

/// Handle user connection
///
/// Takes user's stream as first argument and hashmap in which all keys are saved as second
/// Transforms json format to String -> checks request -> sends answer based on rules:
/// Load request:
/// {
//   "request_type": "load",
//   "key": "some_key"
// }
/// Answer:
/// {
///   "response_status": "success",
///   "requested_key": "some_key",
///   "requested_hash": "0b672dd94fd3da6a8d404b66ee3f0c83",
/// }
/// Store request:
/// {
/// "request_type": "store",
/// "key": "some_key",
/// "hash": "0b672dd94fd3da6a8d404b66ee3f0c83"
/// }
/// Answer:
/// {
///   "response_status": "success",
/// }
///
fn handle_connection(mut stream: TcpStream, data: &mut HashMap<String, String>) -> io::Result<()> {
    let mut reader = BufReader::new(&mut stream);
    let mut buffer = vec![];

    reader.read_until(b'}', &mut buffer).expect("Fail of reading");
    let parsed_string = std::str::from_utf8(&buffer).unwrap();
    let req: Value = serde_json::from_str(parsed_string)?;

    let request_type = req["request_type"].as_str().unwrap().to_string();
    let key = req["key"].as_str().unwrap().to_string();

    if request_type == "store" {
        let hash_of_key = req["hash"].as_str().unwrap().to_string();
        println!(
            "{:?} Received request to write new value {} by key {}. Storage size: {}",
            stream.peer_addr(),
            hash_of_key.clone(),
            key.clone(),
            data.len() + 1
        );
        data.insert(
            key.clone(),
            hash_of_key.clone(),
        );
        let response = SuccessOrFailResponse {
            response_status: "success".to_string(),
        };
        let response_to_json = serde_json::to_string(&response)?;
        stream.write_all(response_to_json.as_bytes()).expect("Fail of sending response to client");
    } else if request_type == "load" {
        println!(
            "{:?} Received request to get value {}. Storage size: {}",
            stream.peer_addr(),
            key.clone(),
            data.len()
        );
        if !data
            .get(&key)
            .is_none()
        {
            let response = LoadResponse {
                response_status: "success".to_string(),
                requested_key: key.clone(),
                requested_hash: data
                    .get(&key.clone())
                    .unwrap()
                    .to_string(),
            };
            let response_to_json = serde_json::to_string(&response)?;
            stream.write_all(response_to_json.as_bytes()).expect("Fail of sending response to client");
        } else {
            let response = SuccessOrFailResponse {
                response_status: "key not found".to_string(),
            };
            let response_to_json = serde_json::to_string(&response)?;
            stream.write_all(response_to_json.as_bytes()).expect("Fail of sending response to client");
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7777").unwrap();
    let mut data = HashMap::new();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!(
            "{:?} Connection established. Storage size: {}",
            stream.peer_addr(),
            data.len()
        );
        handle_connection(stream, &mut data)?;
        println!("Connection is here");
    }

    Ok(())
}
