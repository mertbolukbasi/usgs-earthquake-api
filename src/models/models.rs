use serde::{Deserialize, Serialize};


/// Root response object from the USGS Earthquake API.
///
/// Contains metadata, bounding box, and a list of earthquake features.
#[derive(Deserialize, Debug)]
pub struct EarthquakeResponse {

	/// Response type (always `"FeatureCollection"` for GeoJSON).
	#[serde(rename = "type")]
	pub data_type: String,

	/// List of earthquake features (events).
	pub features: Vec<EarthquakeFeatures>,

	/// Metadata about the request and response.
	pub metadata: EarthquakeMetadata,

	/// Optional bounding box of the returned dataset.
	pub bbox: Option<Vec<f32>>
}


/// Metadata returned by the USGS Earthquake API.
///
/// Includes API version, request information, and count of features.
#[derive(Deserialize, Debug)]
pub struct EarthquakeMetadata {

	/// Unix timestamp when the data was generated.
	#[serde(rename = "generated")]
	pub generated_timestamp: u64,

	/// URL of the API request.
	#[serde(rename = "url")]
	pub url: String,

	/// Human-readable title of the dataset.
	#[serde(rename = "title")]
	pub title: String,

	/// HTTP-like status code of the response.
	#[serde(rename = "status")]
	pub status: u32,

	/// Version of the API used.
	#[serde(rename = "api")]
	pub api_version: String,

	/// Number of earthquake events returned.
	#[serde(rename = "count")]
	pub count: u32
}


/// Represents a single earthquake feature (event).
#[derive(Deserialize, Debug)]
pub struct EarthquakeFeatures {

	/// Feature type (usually `"Feature"`).
	#[serde(rename = "type")]
	pub feature_type: String,

	/// Properties of the earthquake (magnitude, location, etc.).
	pub properties: EarthquakeProperties,

	/// Geometric information (coordinates).
	pub geometry: EarthquakeGeometry,

	/// Unique identifier for the earthquake.
	pub id: String
}


/// Detailed properties of an earthquake event.
///
/// All fields are optional since not every event provides complete data.
#[derive(Serialize, Deserialize, Debug)]
pub struct EarthquakeProperties {
	/// Magnitude of the earthquake.
	#[serde(rename = "mag")]
	pub magnitude: Option<f64>,

	/// Location description (e.g., `"10km NE of City"`).
	#[serde(rename = "place")]
	pub place: Option<String>,

	/// Event timestamp (milliseconds since Unix epoch).
	#[serde(rename = "time")]
	pub time: Option<u64>,

	/// Last updated timestamp.
	#[serde(rename = "updated")]
	pub updated_time: Option<u64>,

	/// Timezone offset in minutes.
	#[serde(rename = "tz")]
	pub tz: Option<i32>,

	/// Event detail URL.
	#[serde(rename = "url")]
	pub url: Option<String>,

	/// Detailed event API URL.
	#[serde(rename = "detail")]
	pub detail: Option<String>,

	/// Number of people who reported feeling the event.
	#[serde(rename = "felt")]
	pub felt: Option<u32>,

	/// Community Internet Intensity (perceived shaking).
	#[serde(rename = "cdi")]
	pub cdi: Option<f32>,

	/// Modified Mercalli Intensity (measured shaking).
	#[serde(rename = "mmi")]
	pub mmi: Option<f32>,

	/// Alert level (`green`, `yellow`, `orange`, `red`).
	#[serde(rename = "alert")]
	pub alert_level: Option<String>,

	/// Status of the event (`reviewed`, `automatic`, etc.).
	#[serde(rename = "status")]
	pub status: Option<String>,

	/// Whether the earthquake triggered a tsunami (0 = no, 1 = yes).
	#[serde(rename = "tsunami")]
	pub tsunami: Option<u8>,

	/// Significance index of the earthquake.
	#[serde(rename = "sig")]
	pub sig: Option<u32>,

	/// Network identifier.
	#[serde(rename = "net")]
	pub net: Option<String>,

	/// Event code within the network.
	#[serde(rename = "code")]
	pub code: Option<String>,

	/// List of event IDs from multiple sources.
	#[serde(rename = "ids")]
	pub ids: Option<String>,

	/// Sources contributing to the event.
	#[serde(rename = "sources")]
	pub sources: Option<String>,

	/// Types of reported data (`origin`, `phase-data`, etc.).
	#[serde(rename = "types")]
	pub types: Option<String>,

	/// Number of seismic stations used.
	#[serde(rename = "nst")]
	pub nst: Option<u32>,

	/// Minimum distance to a station (degrees).
	#[serde(rename = "dmin")]
	pub dmin: Option<f64>,

	/// Root-mean-square travel time residual.
	#[serde(rename = "rms")]
	pub rms: Option<f32>,

	/// Azimuthal gap between stations.
	#[serde(rename = "gap")]
	pub gap: Option<u32>,

	/// Type of magnitude used (e.g., `"mb"`, `"ml"`).
	#[serde(rename = "magType")]
	pub magnitude_type: Option<String>,

	/// Event type (`earthquake`, `quarry blast`, etc.).
	#[serde(rename = "type")]
	pub event_type: Option<String>,

	/// Title for the event (often a combination of magnitude + place).
	#[serde(rename = "title")]
	pub title: Option<String>,
}

/// Geometric data for an earthquake event.
///
/// Contains coordinates and geometry type.
#[derive(Deserialize, Debug)]
pub struct EarthquakeGeometry {
	/// Geometry type (always `"Point"` for earthquakes).
	#[serde(rename = "type")]
	pub geometry_type: String,

	/// Coordinates in `[longitude, latitude, depth]` order.
	#[serde(rename = "coordinates")]
	pub coordinates: Vec<f32>,
}