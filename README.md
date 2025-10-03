# usgs-earthquake-api

[![Crates.io](https://img.shields.io/crates/v/usgs-earthquake-api.svg)](https://crates.io/crates/usgs-earthquake-api)
[![Docs.rs](https://docs.rs/usgs-earthquake-api/badge.svg)](https://docs.rs/usgs-earthquake-api)

A Rust client library for querying the **[USGS Earthquake API](https://earthquake.usgs.gov/fdsnws/event/1/)**.  
It provides request validation, error handling, and data models for earthquake events.  
You can filter by time, magnitude, alert level, ordering, and even by country boundaries.

---

## ✨ Features

- Query the USGS Earthquake API with customizable filters
- Convert local times to UTC automatically
- Validate input parameters (start/end time, magnitudes, etc.)
- Filter earthquake results by country code (using [`country-boundaries`](https://crates.io/crates/country-boundaries))
- Strongly typed response models (`EarthquakeResponse`, `EarthquakeFeatures`, `EarthquakeProperties`)
- Rich error handling with `thiserror::Error`

---

## 🚀 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
usgs-earthquake-api = "0.1.0"
```

---

## 📜 License

This project is licensed under the [MIT License](LICENSE).

---

## 👨‍💻 Author

**Mert Bölükbaşı**

Founder of Kaizenium Foundation  
Email: kaizenium@kaizeniumfoundation.com