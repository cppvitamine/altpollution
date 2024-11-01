
# altpollution

**altpollution** is a real-time air quality monitoring project utilizing the PMS7003 air quality sensor. Built in Rust, the project gathers air quality data, stores it locally, and publishes it in a format compatible with any software respecting the defined gRPC data structure.

## Project Structure

The core components of **altpollution** are organized as follows:

### 1. Hardware Interface
   - Acts as an abstraction layer, akin to a simplified Hardware Abstraction Layer (HAL).
   - Through **Adapters** (instances of transceivers), this component manages data collection and storage processes.
   - Data from sensors is stored in UnQLite on dedicated topics, each uniquely serialized to ensure consistency.

### 2. Transceiver
   - Directly interacts with the PMS7003 sensor.
   - Collects raw air quality data from the sensor, processing and framing it as a gRPC object.
   - Provides a gRPC-compatible serialized output for seamless integration with other compatible software.

## Data Serialization for IPC and Storage

- **gRPC** is used for data serialization, providing a structured and universally recognizable format.
- Any other piece of software adhering to this gRPC data structure can read and parse the stored data, supporting seamless data sharing across platforms.
- **UnQLite** is the embedded NoSQL key-value lightweight database, to store gRPC serialized data on its relative topic

## Tech Stack

- **Programming Language**: Rust
- **Database**: UnQLite (for local data key-value storage on relative topic)
- **IPC and Data Serialization**: gRPC (for universally compatible data sharing)
- **Hardware**: PMS7003 Air Quality Sensor
