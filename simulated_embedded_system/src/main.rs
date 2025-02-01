use rand::Rng;
use std::fs::OpenOptions;
use std::io::{self, Read, Write, Result};
use chrono::Local;
use std::thread;
use std::time::{Duration, Instant};
use serde::Deserialize;
use std::fs;
use std::error::Error;

// Configuration structure
#[derive(Debug, Deserialize)]
struct Config {
    sensors: SensorConfig,
    storage: StorageConfig,
    display: DisplayConfig,
    alerts: AlertConfig,
}

#[derive(Debug, Deserialize)]
struct SensorConfig {
    temperature_sampling_rate: u64,
    humidity_sampling_rate: u64,
    light_sampling_rate: u64,
}

#[derive(Debug, Deserialize)]
struct StorageConfig {
    log_file_path: String,
}

#[derive(Debug, Deserialize)]
struct DisplayConfig {
    real_time_graph: bool,
}

#[derive(Debug, Deserialize)]
struct AlertConfig {
    temperature_threshold: f32,
    humidity_threshold: f32,
    light_threshold: f32,
}

impl Config {
    fn load(path: &str) -> std::result::Result<Self, Box<dyn Error>> {
        let config_str = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&config_str)?;
        Ok(config)
    }
}

// Simulated Virtual Sensor
struct VirtualSensor;

impl VirtualSensor {
    fn new() -> Self {
        VirtualSensor
    }

    fn read_temperature(&self) -> f32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(20.0..30.0) // Simulate temperature between 20°C and 30°C
    }

    fn read_humidity(&self) -> f32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(30.0..70.0) // Simulate humidity between 30% and 70%
    }

    fn read_light_intensity(&self) -> f32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(0.0..100.0) // Simulate light intensity between 0% and 100%
    }
}

// Console Display
struct ConsoleDisplay;

impl ConsoleDisplay {
    fn new() -> Self {
        ConsoleDisplay
    }

    fn show(&self, data: &str) {
        println!("[LCD Display]: {}", data);
    }

    fn show_graph(&self, values: &[f32]) {
        println!("[Real-Time Graph]");
        for &value in values {
            let bar = "=".repeat(value as usize / 2); // Scale for better visualization
            println!("{:5.2} | {}", value, bar);
        }
    }

    fn show_alert(&self, message: &str) {
        println!("[ALERT]: {}", message);
    }
}

// File Logger
struct FileLogger {
    file_path: String,
}

impl FileLogger {
    fn new(file_path: &str) -> Self {
        FileLogger {
            file_path: file_path.to_string(),
        }
    }

    fn log(&self, data: &str) -> Result<()> {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.file_path)?;

        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        writeln!(file, "{}, {}", timestamp, data)?;

        Ok(())
    }
}

// Statistics utility
fn calculate_statistics(values: &[f32]) -> (f32, f32, f32) {
    let sum: f32 = values.iter().sum();
    let count = values.len() as f32;
    let average = sum / count;
    let min = values.iter().cloned().fold(f32::INFINITY, f32::min);
    let max = values.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    (average, min, max)
}

// Alert utility
fn check_alerts(temperature: f32, humidity: f32, light: f32, config: &AlertConfig) -> Vec<String> {
    let mut alerts = Vec::new();
    if temperature > config.temperature_threshold {
        alerts.push(format!("Temperature exceeded threshold: {:.2}°C", temperature));
    }
    if humidity > config.humidity_threshold {
        alerts.push(format!("Humidity exceeded threshold: {:.2}%", humidity));
    }
    if light > config.light_threshold {
        alerts.push(format!("Light intensity exceeded threshold: {:.2}%", light));
    }
    alerts
}

// Clear the screen
fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

// Run the simulation
fn run_simulation(config: &Config) -> std::result::Result<(), Box<dyn Error>> {
    let sensor = VirtualSensor::new();
    let display = ConsoleDisplay::new();
    let logger = FileLogger::new(&config.storage.log_file_path);
    let mut temperature_values: Vec<f32> = Vec::new();

    // Ask the user for simulation duration
    println!("Choose simulation duration:");
    println!("1. 15 seconds");
    println!("2. 30 seconds");
    print!("> ");
    io::stdout().flush()?;

    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;

    let duration_seconds = match choice.trim() {
        "1" => 15,
        "2" => 30,
        _ => {
            println!("Invalid choice! Defaulting to 30 seconds.");
            30
        }
    };

    println!("Starting simulation for {} seconds...", duration_seconds);
    let start_time = Instant::now();
    let mut last_temperature_time = Instant::now();
    let mut last_humidity_time = Instant::now();
    let mut last_light_time = Instant::now();

    loop {
        // Check if the duration has elapsed
        if start_time.elapsed() >= Duration::from_secs(duration_seconds) {
            println!("Simulation completed after {} seconds.", duration_seconds);
            break;
        }

        // Read sensor data based on sampling rates
        let mut temperature = None;
        let mut humidity = None;
        let mut light = None;

        if last_temperature_time.elapsed() >= Duration::from_secs(config.sensors.temperature_sampling_rate) {
            temperature = Some(sensor.read_temperature());
            last_temperature_time = Instant::now();
        }

        if last_humidity_time.elapsed() >= Duration::from_secs(config.sensors.humidity_sampling_rate) {
            humidity = Some(sensor.read_humidity());
            last_humidity_time = Instant::now();
        }

        if last_light_time.elapsed() >= Duration::from_secs(config.sensors.light_sampling_rate) {
            light = Some(sensor.read_light_intensity());
            last_light_time = Instant::now();
        }

        // Display sensor data
        if let (Some(temp), Some(hum), Some(lgt)) = (temperature, humidity, light) {
            display.show(&format!(
                "Temperature: {:.2}°C, Humidity: {:.2}%, Light: {:.2}%",
                temp, hum, lgt
            ));

            // Log sensor data to a file
            logger
                .log(&format!("{:.2}, {:.2}, {:.2}", temp, hum, lgt))
                .expect("Failed to log data");

            // Update and display real-time graph
            temperature_values.push(temp);
            if temperature_values.len() > 10 {
                temperature_values.remove(0); // Keep only the last 10 values
            }
            if config.display.real_time_graph {
                display.show_graph(&temperature_values);
            }

            // Check for alerts
            let alerts = check_alerts(temp, hum, lgt, &config.alerts);
            for alert in alerts {
                display.show_alert(&alert);
            }

            // Calculate and display statistics
            let (avg, min, max) = calculate_statistics(&temperature_values);
            println!("[Statistics] Average: {:.2}°C, Min: {:.2}°C, Max: {:.2}°C", avg, min, max);
        }

        // Simulate a delay (e.g., 1 second)
        thread::sleep(Duration::from_secs(1));
    }
    Ok(())
}

// Adjust settings
fn adjust_settings(config: &mut Config) -> std::result::Result<(), Box<dyn Error>> {
    loop {
        clear_screen();
        println!("=== Current Settings ===");
        println!("1. Temperature Sampling Rate: {}s", config.sensors.temperature_sampling_rate);
        println!("2. Humidity Sampling Rate: {}s", config.sensors.humidity_sampling_rate);
        println!("3. Light Sampling Rate: {}s", config.sensors.light_sampling_rate);
        println!("4. Temperature Alert Threshold: {}°C", config.alerts.temperature_threshold);
        println!("5. Humidity Alert Threshold: {}%", config.alerts.humidity_threshold);
        println!("6. Light Alert Threshold: {}%", config.alerts.light_threshold);
        println!("7. Back to Main Menu");
        print!("> ");
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "1" => {
                println!("Enter new Temperature Sampling Rate (in seconds):");
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                config.sensors.temperature_sampling_rate = input.trim().parse()?;
            }
            "2" => {
                println!("Enter new Humidity Sampling Rate (in seconds):");
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                config.sensors.humidity_sampling_rate = input.trim().parse()?;
            }
            "3" => {
                println!("Enter new Light Sampling Rate (in seconds):");
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                config.sensors.light_sampling_rate = input.trim().parse()?;
            }
            "4" => {
                println!("Enter new Temperature Alert Threshold (in °C):");
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                config.alerts.temperature_threshold = input.trim().parse()?;
            }
            "5" => {
                println!("Enter new Humidity Alert Threshold (in %):");
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                config.alerts.humidity_threshold = input.trim().parse()?;
            }
            "6" => {
                println!("Enter new Light Alert Threshold (in %):");
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                config.alerts.light_threshold = input.trim().parse()?;
            }
            "7" => break,
            _ => println!("Invalid choice!"),
        }
    }
    Ok(())
}

// View logs
fn view_logs(log_path: &str) -> std::result::Result<(), Box<dyn Error>> {
    clear_screen();
    println!("=== Sensor Logs ===");
    let logs = fs::read_to_string(log_path)?;
    println!("{}", logs);
    println!("Press Enter to continue...");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(())
}

// Main Function with Menu
fn main() -> std::result::Result<(), Box<dyn Error>> {
    let mut config = Config::load("config.toml")?;

    loop {
        clear_screen();
        println!("=== Embedded System Simulator ===");
        println!("1. Start Simulation");
        println!("2. Adjust Settings");
        println!("3. View Logs");
        println!("4. Exit");
        print!("> ");
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "1" => run_simulation(&config)?,
            "2" => adjust_settings(&mut config)?,
            "3" => view_logs(&config.storage.log_file_path)?,
            "4" => break,
            _ => println!("Invalid choice!"),
        }
    }

    println!("Exiting...");
    Ok(())
}