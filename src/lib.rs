//! # Usgs Earthquake API
//!
//! This crate provides a simple client for interacting with the
//! [USGS Earthquake API](https://earthquake.usgs.gov/fdsnws/event/1/).
//!
//! Features:
//! - Filter earthquakes by time range (`start_time`, `end_time`)
//! - Filter by magnitude range (`min_magnitude`, `max_magnitude`)
//! - Filter by alert level (`AlertLevel`)
//! - Order results (`OrderBy`)
//! - Filter earthquakes by country code (using `country_boundaries` dataset).
//!
//! ## Example
//! ```rust,no_run
//! use usgs_client::{UsgsClient, AlertLevel, OrderBy};
//!
//! #[tokio::main]
//! async fn main() {
//!     use usgs_earthquake_api::OrderBy;
//! let client = UsgsClient::new();
//!     let result = client
//!         .query()
//!         .filter_by_country_code("TR")
//!         .start_time(2024, 1, 1, 0, 0)
//!         .end_time(2024, 12, 31, 23, 59)
//!         .min_magnitude(4.0)
//!         .alert_level(AlertLevel::All)
//!         .order_by(OrderBy::Time)
//!         .fetch()
//!         .await;
//!
//!     match result {
//!         Ok(res) => println!("Total earthquakes: {}", res.features.len()),
//!         Err(e) => eprintln!("Error: {}", e),
//!     }
//! }
//! ```

mod error;
mod models;

use std::fmt::Display;
use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use country_boundaries::{CountryBoundaries, LatLon, BOUNDARIES_ODBL_360X180};
use reqwest::Client;
use error::error::UsgsError;
use crate::models::models::{EarthquakeResponse, EarthquakeFeatures};

fn local_time_as_utc() -> NaiveDateTime {
	Utc::now().naive_utc()
}

fn local_time_to_utc(time: NaiveDateTime) -> NaiveDateTime {
	let timezone = Local.from_local_datetime(&time).unwrap();
	let utc = timezone.with_timezone(&Utc);
	println!("{}", utc.naive_utc().to_string());
	utc.naive_utc()
}

fn generate_custom_time(year: i32, month: u32, day: u32, hour: u32, min: u32) -> NaiveDateTime {
	let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
	let time = NaiveTime::from_hms_opt(hour, min, 00).unwrap();
	NaiveDateTime::new(date, time)
}


/// USGS earthquake alert levels.
#[derive(Debug)]
pub enum AlertLevel {
	/// Low alert level
	Green,

	/// Moderate alert level
	Yellow,

	/// High alert level
	Orange,

	/// Very high alert level
	Red,

	/// All alert levels
	All
}

pub enum OrderBy {
	/// Order by time descending
	Time,

	/// Order by time ascending
	TimeAsc,

	/// Order by magnitude descending
	Magnitude,

	/// Order by magnitude ascending
	MagnitudeAsc
}


/// Main USGS API client.
///
/// Handles API requests and creates queries.
pub struct UsgsClient {
	/// Base URL of the USGS API
	pub base_url: String,

	/// HTTP client
	pub client: Client,
}


impl UsgsClient {
	/// Creates a new [`UsgsClient`].
	pub fn new() -> Self {
		Self {
			base_url: "https://earthquake.usgs.gov/fdsnws/event/1/query?format=geojson".to_string(),
			client: Client::new(),
		}
	}

	/// Starts a new [`UsgsQuery`] with default parameters.
	pub fn query(&self) -> UsgsQuery<'_> {
		UsgsQuery {
			client: &self.client,
			base_url: self.base_url.clone(),
			country_code: "US".to_string(),
			start_time: None,
			end_time: local_time_as_utc(),
			min_magnitude: 0.0,
			max_magnitude: 10.0,
			alert_level: AlertLevel::All,
			order_by: OrderBy::Time,
		}
	}
}

/// Query builder for the USGS API.
///
/// Allows filtering and customizing request parameters.
pub struct UsgsQuery<'a> {
	client: & 'a Client,
	base_url: String,
	country_code: String,
	start_time: Option<NaiveDateTime>,
	end_time: NaiveDateTime,
	min_magnitude: f32,
	max_magnitude: f32,
	alert_level: AlertLevel,
	order_by: OrderBy,
}

//TODO: Add other queries from USGS API document.
impl<'a> UsgsQuery<'a> {

	/// Filters earthquakes by country code (e.g., `"TR"`, `"US"`).
	pub fn filter_by_country_code(mut self, country_code: &str) -> Self {
		self.country_code = country_code.to_string();
		self
	}

	/// Sets the start time for the query.
	pub fn start_time(mut self, year: i32, month: u32, day: u32, hour: u32, min: u32) -> Self {
		self.start_time =  Some(local_time_to_utc(generate_custom_time(year, month, day, hour, min)));
		self
	}

	/// Sets the end time for the query.
	pub fn end_time(mut self, year: i32, month: u32, day: u32, hour: u32, min: u32) -> Self {
		self.end_time = local_time_to_utc(generate_custom_time(year, month, day, hour, min));
		self
	}

	/// Sets the minimum magnitude filter.
	pub fn min_magnitude(mut self, min: f32) -> Self {
		self.min_magnitude = min;
		self
	}

	/// Sets the maximum magnitude filter.
	pub fn max_magnitude(mut self, max: f32) -> Self {
		self.max_magnitude = max;
		self
	}

	/// Sets the alert level filter.
	pub fn alert_level(mut self, level: AlertLevel) -> Self {
		self.alert_level = level;
		self
	}

	/// Sets the ordering method for the query.
	pub fn order_by(mut self, order_by: OrderBy) -> Self {
		self.order_by = order_by;
		self
	}

	/// Executes the query against the USGS API.
	///
	/// # Returns
	/// `Result<EarthquakeResponse, UsgsError>`
	pub async fn fetch(self) -> Result<EarthquakeResponse, UsgsError> {

		if self.start_time.is_none() {
			return Err(UsgsError::EmptyStartTime)
		}

		let start_time = self.start_time.unwrap();

		if start_time > self.end_time {
			return Err(UsgsError::InvalidStartTime);
		}

		if start_time > local_time_as_utc() {
			return Err(UsgsError::StartTimeInFuture)
		}
		
		if self.min_magnitude < 0.0 {
			return Err(UsgsError::MinimumMagnitude)
		}
		
		if self.max_magnitude > 10.0 {
			return Err(UsgsError::MaximumMagnitude)
		}


		let mut url = format!("{}&starttime={}&endtime={}&minmagnitude={}&maxmagnitude={}&alertlevel={}&orderby={}"
		                     ,self.base_url, start_time, self.end_time, self.min_magnitude, self.max_magnitude, self.alert_level.to_string(), self.order_by.to_string());

		if self.alert_level.to_string() == "all" {
			url = format!("{}&starttime={}&endtime={}&minmagnitude={}&maxmagnitude={}&orderby={}"
			                  ,self.base_url, start_time.and_utc(), self.end_time, self.min_magnitude, self.max_magnitude, self.order_by.to_string());
		}

		let response = self.client.get(&url).send().await?;
		let mut body: EarthquakeResponse = response.json().await?;
		if !self.country_code.is_empty() {
			let boundaries = CountryBoundaries::from_reader(BOUNDARIES_ODBL_360X180).expect("Failed to parse BOUNDARIES_ODBL_360X180");
			let target_code = &self.country_code;
			let filtered_features: Vec<EarthquakeFeatures> = body.features.into_iter()
				.filter(|eq| {
					let coordinates = &eq.geometry.coordinates;
					let lon = coordinates[0] as f64;
					let lat = coordinates[1] as f64;
					let country_codes = boundaries.ids(LatLon::new(lat, lon).expect("Failed to parse LatLon"));
					country_codes.contains(&&**target_code)
				})
			.collect();

			body.features = filtered_features;
			body.metadata.count = body.features.len() as u32;
		}
		Ok(body)

	}
}

impl Display for AlertLevel {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let level = match self {
			AlertLevel::Green => "green",
			AlertLevel::Yellow => "yellow",
			AlertLevel::Orange => "orange",
			AlertLevel::Red => "red",
			AlertLevel::All => "all"
		};
		write!(f, "{}", level)
	}
}


impl Display for OrderBy {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s = match self {
			OrderBy::Time => "time",
			OrderBy::TimeAsc => "time-asc",
			OrderBy::Magnitude => "magnitude",
			OrderBy::MagnitudeAsc => "magnitude-asc",
		};
		write!(f, "{}", s)
	}
}