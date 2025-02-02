use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time;

#[derive(Serialize, Deserialize)]
struct AuthResponse {
    key: Vec<String>,
    value: AuthResponseValue,
    versionstamp: String,
}

#[derive(Serialize, Deserialize)]
struct AuthResponseValue {
    token: String,
    expires_at: i64,
}

fn get_config_path() -> String {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    format!("{}/.kumulus_auth.json", home)
}

pub fn login() {
    let client = Client::builder()
        .build()
        .expect("Failed to create HTTP client");

    let base_url = "https://test-kumulus-backend.deno.dev/auth"; // "http://localhost:8000/auth";
    let state = uuid::Uuid::new_v4().to_string();
    let auth_url = format!("{}{}{}", base_url, "/cli-portal?state=", state);

    println!("Opening browser for authentication: {}", auth_url);

    if let Err(e) = open_browser(&auth_url) {
        println!(
            "Failed to open browser automatically: {}. Please visit this URL manually: {}",
            e, auth_url
        );
    }

    println!("Waiting for authentication...");

    let wait_time = time::Duration::from_secs(2);
    let max_duration = time::Duration::from_secs(60);
    let start_time = time::Instant::now();

    while start_time.elapsed() < max_duration {
        let check_url = format!("{}{}{}", base_url, "/cli-token?state=", state);

        match client.get(&check_url).send() {
            Ok(response) if response.status().is_success() => {
                println!(
                    "Received successful response with status: {}",
                    response.status()
                );

                match response.text() {
                    Ok(text) => {
                        println!("Raw response body: {}", text);

                        // Try to parse the text back into JSON
                        match serde_json::from_str::<AuthResponse>(&text) {
                            Ok(auth_data) => {
                                let config_path = get_config_path();
                                match fs::write(
                                    &config_path,
                                    serde_json::to_string(&auth_data).unwrap(),
                                ) {
                                    Ok(_) => {
                                        println!(
                                            "Successfully logged in! Token saved to {}",
                                            config_path
                                        );
                                        return;
                                    }
                                    Err(e) => {
                                        println!("Error saving authentication data: {}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                println!("Failed to parse auth response: {}", e);
                                println!("Response was: {}", text);
                            }
                        }
                    }
                    Err(e) => {
                        println!("Failed to get response text: {}", e);
                    }
                }
            }
            Ok(response) => {
                println!("Received non-success response: {}", response.status());
                match response.text() {
                    Ok(text) => println!("Response body: {}", text),
                    Err(_) => println!("Could not read response body"),
                }
            }
            Err(err) => {
                println!("Network error while checking authentication: {}", err);
            }
        }
        thread::sleep(wait_time);
    }

    println!("Login timeout. Please try again.");
}

pub fn logout() {
    let config_path = get_config_path();
    let path = Path::new(&config_path);

    if path.exists() {
        if let Err(e) = fs::remove_file(path) {
            println!("Error removing authentication file: {}", e);
        } else {
            println!("Logged out successfully.");
        }
    } else {
        println!("No active session found.");
    }
}

fn open_browser(url: &str) -> std::io::Result<()> {
    // Try wslview (for devs who have linux with wsl)
    if Command::new("wslview").arg(url).spawn().is_ok() {
        println!("Successfully opened browser using wslview");
        return Ok(());
    }

    // Fall back to other methods if wslview fails
    let is_wsl = Path::new("/proc/version").exists()
        && fs::read_to_string("/proc/version")
            .map(|s| s.contains("Microsoft"))
            .unwrap_or(false);

    if is_wsl {
        let powershell_command = format!("start '{}'", url);

        let commands = [
            (
                "/mnt/c/Windows/System32/WindowsPowerShell/v1.0/powershell.exe",
                vec!["-c", &powershell_command],
            ),
            ("/mnt/c/Windows/System32/cmd.exe", vec!["/c", "start", url]),
            ("/mnt/c/Windows/explorer.exe", vec![url]),
        ];

        for (cmd, args) in &commands {
            if Path::new(cmd).exists() {
                if Command::new(cmd).args(args).spawn().is_ok() {
                    println!("Successfully opened browser using {}", cmd);
                    return Ok(());
                }
            }
        }
    }

    // Fall back to normal OS detection for non-WSL environments
    let cmd = if cfg!(target_os = "linux") {
        "xdg-open"
    } else if cfg!(target_os = "macos") {
        "open"
    } else if cfg!(target_os = "windows") {
        "cmd"
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Unsupported OS",
        ));
    };

    let mut command = Command::new(cmd);
    if cfg!(target_os = "windows") {
        command.args(&["/C", "start", url]);
    } else {
        command.arg(url);
    }

    command.spawn()?;
    Ok(())
}
