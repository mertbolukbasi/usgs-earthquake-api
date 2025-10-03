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

# 🔧 Example usage

```rust
use usgs_earthquake_api::{UsgsClient, AlertLevel, OrderBy};
use tokio;

#[tokio::main]
async fn main() {
    let client = UsgsClient::new();

    let result = client
        .query()
        .filter_by_country_code("TR")
        .start_time(2024, 12, 1, 0, 0)
        .end_time(2024, 12, 31, 23, 59)
        .min_magnitude(5.0)
        .order_by(OrderBy::Magnitude)
        .alert_level(AlertLevel::All)
        .fetch()
        .await;

    match result {
        Ok(response) => {
            println!("Found {} earthquakes", response.metadata.count);
            for eq in response.features {
                println!(
                    "Magnitude: {:?}, Location: {:?}",
                    eq.properties.magnitude,
                    eq.properties.place
                );
            }
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}
```

---

# 📊 Data Source & Disclaimer

This crate uses the **[USGS Earthquake API](https://earthquake.usgs.gov/fdsnws/event/1/)**.
The data is provided by the United States Geological Survey (USGS).

⚠️ Disclaimer:

USGS provides this data as a public service. No warranty is expressed or implied regarding
accuracy, timeliness, or completeness of the data. See the **[USGS Website](https://www.usgs.gov/)**.

This library is an unofficial client and is not affiliated with or endorsed by the USGS.

---

## 📜 License

This project is licensed under the [MIT License](LICENSE).

---

## 👨‍💻 Author

**Mert Bölükbaşı**

Founder of Kaizenium Foundation  
Email: kaizenium@kaizeniumfoundation.com

---
