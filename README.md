# serde_practice

## Description

This project demonstrates the use of the serde library for serializing and deserializing structures in Rust. The project implements event processing (Event) with support for custom serialization/deserialization to handle dates.

## Features

	•	Serialization and deserialization of the Event structure in JSON format.
	•	Use of serde to handle custom serialization and deserialization functions.
	•	Demonstrates functionality with JSON, YAML, and TOML formats.

## Dependencies

The project uses the following dependencies:

```toml
[dependencies]
serde = { version = "1.0.214", features = ["derive"] }
serde_json = "1.0.132"
serde_yaml = "0.9.34"
toml = "0.8.19"
