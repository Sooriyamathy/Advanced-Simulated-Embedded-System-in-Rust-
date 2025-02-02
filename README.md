---

# **Simulated Embedded System Using Rust** 🦀

![Rust Logo](https://www.rust-lang.org/static/images/rust-logo-blk.svg)

This project simulates an embedded system using Rust. It demonstrates how Rust can be used for embedded systems development by simulating virtual sensors, logging data, displaying real-time statistics, and triggering alerts based on predefined thresholds.

---

## **Table of Contents**
1. [Introduction](#introduction)
2. [Features](#features)
3. [System Design](#system-design)
4. [Installation](#installation)
5. [Usage](#usage)
6. [Configuration](#configuration)
7. [Rust vs. C Comparison](#rust-vs-c-comparison)
8. [Advantages of Using Rust](#advantages-of-using-rust)
9. [Future Work](#future-work)
10. [Contributing](#contributing)
11. [License](#license)

---

## **Introduction** 📖

This project simulates an embedded system with virtual sensors for temperature, humidity, and light intensity. It showcases Rust's capabilities in embedded systems development, including memory safety, modern tooling, and high performance. The system logs sensor data, displays real-time statistics, and triggers alerts when thresholds are exceeded.

---

## **Features** ✨

- **Virtual Sensors**:
  - Simulate temperature, humidity, and light intensity readings.
- **Data Logging**:
  - Log sensor data to a file with timestamps.
- **Real-Time Display**:
  - Display sensor data and a real-time graph.
- **Alert System**:
  - Trigger alerts if sensor values exceed predefined thresholds.
- **Statistics**:
  - Calculate and display average, minimum, and maximum values.

---

## **System Design** 🛠️

The system is designed with the following components:
- **Configuration Manager**: Loads settings from `config.toml`.
- **Virtual Sensor**: Simulates sensor readings.
- **File Logger**: Logs sensor data to a file.
- **Console Display**: Displays sensor data, graphs, and alerts.
- **Alert System**: Monitors sensor values and triggers alerts.
- **Statistics Calculator**: Computes statistics for sensor data.

---

## **Installation** ⚙️

### **Prerequisites**
- Install [Rust](https://www.rust-lang.org/tools/install).

### **Steps**
1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/simulated-embedded-system-rust.git
   ```
2. Navigate to the project directory:
   ```bash
   cd simulated-embedded-system-rust
   ```
3. Build the project:
   ```bash
   cargo build
   ```

---

## **Usage** 🚀

1. Run the simulation:
   ```bash
   cargo run
   ```
2. Follow the on-screen instructions to choose the simulation duration.
3. View real-time sensor data, graphs, and alerts in the console.
4. Check the log file (`sensor_logs.txt`) for historical data.

---

## **Configuration** ⚡

The system is configured using a `config.toml` file. Here's an example configuration:

```toml
[sensors]
temperature_sampling_rate = 1
humidity_sampling_rate = 2
light_sampling_rate = 3

[storage]
log_file_path = "sensor_logs.txt"

[display]
real_time_graph = true

[alerts]
temperature_threshold = 28.0
humidity_threshold = 65.0
light_threshold = 90.0
```

You can modify the `config.toml` file to adjust sampling rates, thresholds, and other parameters.

---

## **Rust vs. C Comparison** ⚔️

### **Memory Safety**
- **Rust**: Ensures memory safety at compile time using its ownership model.
- **C**: Requires manual memory management, leading to potential issues like null pointer dereferencing.

### **Error Handling**
- **Rust**: Uses the `Result` type for explicit error handling.
- **C**: Relies on manual error checks and return codes.

### **Concurrency**
- **Rust**: Provides thread safety without data races.
- **C**: Requires careful handling of threads and shared resources.

### **Code Readability**
- **Rust**: Modern syntax and abstractions make code more readable.
- **C**: Verbose and prone to boilerplate code.

---

## **Advantages of Using Rust** 🌟

1. **Memory Safety**: Prevents common bugs like null pointers and buffer overflows.
2. **Concurrency**: Ensures thread safety without data races.
3. **Modern Tooling**: Built-in package manager (`Cargo`) and testing framework.
4. **Performance**: Offers C-like performance with modern language features.
5. **Community Support**: Growing ecosystem and active community.

---

## **Future Work** 🔮

1. **Hardware Integration**: Test the system on real hardware (e.g., Raspberry Pi, Arduino).
2. **Performance Benchmarking**: Compare Rust’s performance with C on embedded platforms.
3. **Advanced Features**: Implement wireless communication and cloud integration.

---

## **Contributing** 🤝

Contributions are welcome! If you'd like to contribute, please follow these steps:
1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Commit your changes.
4. Submit a pull request.

---

## **License** 📄

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.

---

## **Acknowledgments** 🙏

- Thanks to the Rust community for creating such an amazing language.
- Inspired by embedded systems projects and tutorials.

---

## **Connect with Me** 🌐

- **LinkedIn**: [Your LinkedIn Profile](https://www.linkedin.com/in/sooriyamathy/)

---

Feel free to explore the code, provide feedback, or contribute to the project! 🚀

---
