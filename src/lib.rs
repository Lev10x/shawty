// src/lib.rs

#![allow(unused)]
// I don't even know if theres a point of doing all this work just to make it panic less.
#![deny(clippy::panic)]
#![deny(clippy::unwrap_used)]

pub mod error;

use crate::error::*;

use std::io::Read;
use std::io::Write;

// A print that won't panic. (Not promise.)
pub fn print_no_panic<T>(msg: &T) -> Result<(), ShawtyError>
where
    T: std::fmt::Display + ?Sized,
{
    let mut lock = std::io::stdout().lock();
    write!(lock, "{msg}").map_err(|e| IOError::WriteError {
        message: "Failed to print. (Failed to write to stdout)".to_string(),
        source: e,
    })?;

    Ok(())
}

// A println that won't panic. (Not promise.)
pub fn println_no_panic<T>(msg: &T) -> Result<(), ShawtyError>
where
    T: std::fmt::Display + ?Sized,
{
    let mut lock = std::io::stdout().lock();
    writeln!(lock, "{msg}").map_err(|e| IOError::WriteError {
        message: "Failed to println (Failed to writeln to stdout)".to_string(),
        source: e,
    })?;

    Ok(())
}

/// Print the provided message then ask for input with, using "" as message will print nothing.
pub fn input(msg: &str) -> Result<String, ShawtyError> {
    if !msg.is_empty() {
        print_no_panic(msg)?;
    };

    std::io::stdout()
        .flush()
        .map_err(|e| IOError::FlushError { source: e })?;

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .map_err(|e| IOError::ReadError {
            message: "Failed to get input (Failed to read stdin)".to_string(),
            source: e,
        })?;

    Ok(input.trim().to_string())
}

/// Hold the console until user pressed `Enter`.
///
/// Print the message if provided, or else print default message: "Press 'Enter' to continue...".
pub fn hold_console(message: Option<&str>) -> Result<(), ShawtyError> {
    if let Some(message) = message {
        println_no_panic(message)?
    } else {
        println_no_panic("Press 'Enter' to continue...")? // Print default message.
    }

    std::io::stdin()
        .read(&mut [0])
        .map_err(|e| IOError::ReadError {
            message: "Failed to hold console. (Failed to read stdin)".to_string(),
            source: e,
        })?;

    Ok(())
}

/// Debug print, print a message with \[DEBUG\] at the beginning and new line at the end.
pub fn dp<T>(msg: &T) -> Result<(), ShawtyError>
where
    T: ToString + ?Sized,
{
    let message = "[DEBUG] ".to_string() + &msg.to_string();
    println_no_panic(&message)?;

    Ok(())
}

/// Get current working directory.
pub fn get_cwd_path() -> Result<std::path::PathBuf, ShawtyError> {
    let cwd_path = std::env::current_dir().map_err(|e| IOError::OtherError {
        message: "Failed to get cwd path, perhaps you don't have permission or it doesn't exist."
            .to_string(),
        source: e,
    })?;

    Ok(cwd_path)
}

/// Get all file and directory from a path.
pub fn list_directory(path: &std::path::Path) -> Result<Vec<std::fs::DirEntry>, ShawtyError> {
    let dir_iter = path.read_dir().map_err(|e| IOError::ReadError {
        message: format!("Failed to read directory: {}", path.to_string_lossy()),
        source: e,
    })?;

    let dir_iter: Vec<std::fs::DirEntry> = dir_iter.flatten().collect();

    Ok(dir_iter)
}

/// Create the directory by the provided path. (If the directory doesnt't exist it will do nothing.)
///
/// Will return Err if the directory does exist after creation.
pub fn create_dir_and_check(path: &std::path::Path) -> Result<(), ShawtyError> {
    if !path
        .try_exists()
        .is_ok_and(|symbol_not_broken| symbol_not_broken)
    {
        std::fs::create_dir(path).map_err(|e| IOError::WriteError {
            message: format!("Failed to create directory: '{}'", path.to_string_lossy()),
            source: e,
        })?;

        if !path
            .try_exists()
            .is_ok_and(|symbol_not_broken| symbol_not_broken)
        {
            return Err(ShawtyError::WeirdError {
                message: format!(
                    "After creation with no error, the directory still not exist. Path: '{}'",
                    path.to_string_lossy()
                ),
            });
        }
    }

    Ok(())
}

/// Clear the console, only on Windows.
#[cfg(target_os = "windows")]
fn clear_console() -> Result<(), ShawtyError> {
    let command = "cmd";
    let command_args = ["/c", "cls"];
    let mut child = std::process::Command::new(command)
        .args(command_args)
        .spawn()
        .map_err(|e| ShawtyError::ExecuteCommandError {
            command: command.to_string(),
            command_args: command_args.iter().map(|f| f.to_string()).collect(),
            spawning_or_executing: "spawning".to_string(),
            error: e,
        })?;

    child.wait().map_err(|e| ShawtyError::ExecuteCommandError {
        command: command.to_string(),
        command_args: command_args.iter().map(|f| f.to_string()).collect(),
        spawning_or_executing: "executing".to_string(),
        error: e,
    })?;

    Ok(())
}

//////////////////////////////////// NETWORK ////////////////////////////////////

/// Create a reqwest client that can be reuse, default timeout is 30 seconds.
#[cfg(feature = "network")]
pub fn create_client(
    timeout_secs: Option<u64>,
    user_agent: Option<&str>,
) -> Result<reqwest::blocking::Client, ShawtyError> {
    let mut builder = reqwest::blocking::ClientBuilder::new()
        .timeout(std::time::Duration::from_secs(timeout_secs.unwrap_or(30)));

    if let Some(ua) = user_agent {
        builder = builder.user_agent(ua);
    };

    let client = builder
        .build()
        .map_err(|e| RequestError::ClientBuilderError { source: e })?;

    Ok(client)
}

/// Send a GET request to the url provided.
///
/// Require a client, recommand using `shawty::create_client()` to create one that can be reuse.
#[cfg(feature = "network")]
pub fn get_request<T>(
    url: &T,
    client: &reqwest::blocking::Client,
) -> Result<reqwest::blocking::Response, ShawtyError>
where
    T: reqwest::IntoUrl + ToString,
{
    let respond = client
        .get(url.as_str())
        .send()
        .map_err(|e| RequestError::RequestFailed {
            url: url.to_string(),
            method: "GET".to_string(),
            source: e,
        })?;

    Ok(respond)
}

/// Send a POST request to the url provided.
///
/// Require a client, recommand using `shawty::create_client()` to create one that can be reuse.
#[cfg(feature = "network")]
pub fn post_request<T>(
    url: &T,
    client: &reqwest::blocking::Client,
    body: Option<String>,
    header: Option<std::collections::HashMap<&str, &str>>,
) -> Result<reqwest::blocking::Response, ShawtyError>
where
    T: reqwest::IntoUrl + ToString + Clone,
{
    let mut request = client.post(url.as_str());

    if let Some(body) = body {
        request = request.body(body)
    }

    if let Some(headers) = header {
        for (name, value) in headers {
            request = request.header(name, value);
        }
    }

    let respond = request.send().map_err(|e| RequestError::RequestFailed {
        url: url.to_string(),
        method: "POST".to_string(),
        source: e,
    })?;

    Ok(respond)
}

/// Send a message to provided Discord webhook url.
///
/// Require a client, recommand using `shawty::create_client()` to create one that can be reuse.
#[cfg(feature = "network")]
pub fn send_to_discord_webhook<T>(
    client: &reqwest::blocking::Client,
    webhook_url: &T,
    message: &str,
) -> Result<reqwest::blocking::Response, ShawtyError>
where
    T: reqwest::IntoUrl + ToString + Clone,
{
    let webhook_url = webhook_url.as_str();
    let url = reqwest::Url::parse(webhook_url).map_err(|e| RequestError::ParseURLError {
        url: webhook_url.to_string(),
        source: e,
    })?;

    if let Some(host_name) = url.host_str() {
        if !host_name.ends_with("discord.com") {
            return Err(RequestError::HostNotMatch {
                input: url.to_string(),
                message: "`send_to_discord_webhook` only accept Discord webhook url.".to_string(),
            }
            .into());
        }
    } else {
        return Err(RequestError::HostNotMatch {
            input: url.to_string(),
            message: "`send_to_discord_webhook` only accept Discord webhook url.".to_string(),
        }
        .into());
    }

    let json_payload = "{\"content\": \"".to_string() + message + "\"}";

    let mut headers = std::collections::hash_map::HashMap::new();
    let _ = headers.insert("Content-Type", "application/json");
    let respond = post_request(&webhook_url, client, Some(json_payload), Some(headers))?;

    Ok(respond)
}

//////////////////////////////////// TESTS ////////////////////////////////////

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn whats_my_user_agent() {
        let url = "https://api.bigdatacloud.net/data/client-info";
        let client = create_client(None, Some("BigData is good.")).unwrap();

        let respond = get_request(&url, &client).unwrap();
        assert_eq!(
            respond.status(),
            200,
            "Get request got respond, but status code is not 200 OK!"
        );

        let respond = respond.text();
        println!("respond: {respond:?}");
        //panic!("Give me stdout")
    }

    #[test]
    fn test_get_cwd_have_same_result_as_std() {
        let my_cwd = get_cwd_path().unwrap();
        let std_cwd = std::env::current_dir().unwrap();

        assert_eq!(
            my_cwd, std_cwd,
            "My function and std function returnd different cwd path!"
        )
    }

    #[test]
    fn test_get_all_file_in_cwd() {
        let entries = list_directory(&get_cwd_path().unwrap()).unwrap();
    }

    #[test]
    #[ignore = "This will clean the unit test console."]
    fn test_clear_console() {
        clear_console().unwrap()
    }

    #[test]
    fn test_get_request() {
        let url = "https://httpbin.org/get";
        let client = create_client(None, None).unwrap();

        let respond = post_request(&url, &client, None, None).unwrap();
        assert_eq!(
            respond.status(),
            405,
            "Post on /get url should get a 405 (Method Not Allowed), but somehow it didn't!"
        );

        let respond = get_request(&url, &client).unwrap();
        assert_eq!(
            respond.status(),
            200,
            "Get request got respond, but status code is not 200 OK!"
        )
    }

    #[test]
    fn test_post_request() {
        let url = "https://httpbin.org/post";
        let client = create_client(None, None).unwrap();
        let respond = get_request(&url, &client).unwrap();
        assert_eq!(
            respond.status(),
            405,
            "GET on /post url should get a 405 (Method Not Allowed), but somehow it didn't!"
        );

        let mut headers = std::collections::hash_map::HashMap::new();
        headers.insert("accept", "application/json");
        let respond = post_request(&url, &client, None, Some(headers)).unwrap();

        assert_eq!(
            respond.status(),
            200,
            "Post request got respond, but status code is not 200 OK!"
        )
    }
}
