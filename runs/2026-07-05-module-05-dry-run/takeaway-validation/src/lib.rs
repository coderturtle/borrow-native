//! Takeaway validation for `custom-error-type-template`: an unrelated domain
//! (retry-policy config parsing), nothing to do with `relay` or checkpoints.

pub struct RetryPolicy {
    pub max_attempts: u32,
    pub backoff_secs: u64,
}

#[derive(Debug)]
pub enum RetryPolicyError {
    MissingField(String),
    InvalidNumber {
        field: String,
        source: std::num::ParseIntError,
    },
}

impl std::fmt::Display for RetryPolicyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RetryPolicyError::MissingField(field) => {
                write!(f, "missing retry policy field: {field}")
            }
            RetryPolicyError::InvalidNumber { field, source } => {
                write!(f, "invalid number for {field}: {source}")
            }
        }
    }
}

impl std::error::Error for RetryPolicyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RetryPolicyError::MissingField(_) => None,
            RetryPolicyError::InvalidNumber { source, .. } => Some(source),
        }
    }
}

/// Parse "max_attempts,backoff_secs" (e.g. "3,5") into a RetryPolicy.
pub fn parse_retry_policy(input: &str) -> Result<RetryPolicy, RetryPolicyError> {
    let mut parts = input.split(',');
    let max_attempts_raw = parts
        .next()
        .ok_or_else(|| RetryPolicyError::MissingField("max_attempts".to_string()))?;
    let backoff_raw = parts
        .next()
        .ok_or_else(|| RetryPolicyError::MissingField("backoff_secs".to_string()))?;

    let max_attempts = max_attempts_raw
        .parse::<u32>()
        .map_err(|source| RetryPolicyError::InvalidNumber {
            field: "max_attempts".to_string(),
            source,
        })?;
    let backoff_secs = backoff_raw
        .parse::<u64>()
        .map_err(|source| RetryPolicyError::InvalidNumber {
            field: "backoff_secs".to_string(),
            source,
        })?;

    Ok(RetryPolicy {
        max_attempts,
        backoff_secs,
    })
}
