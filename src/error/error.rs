use thiserror::Error;

// TODO: Errors can be better
#[derive(Debug, Error)]
pub enum UsgsError {

	#[error("Start time cannot be empty")]
	EmptyStartTime,

	#[error("Start time cannot be after end time")]
	InvalidStartTime,

	#[error("Start time cannot be in the future")]
	StartTimeInFuture,

	#[error("Request error: {0}")]
	Request(#[from] reqwest::Error),

	#[error("Minimum magnitude cannot be smaller than 0")]
	MinimumMagnitude,

	#[error("Maximum magnitude cannot be greater than 10")]
	MaximumMagnitude,
}