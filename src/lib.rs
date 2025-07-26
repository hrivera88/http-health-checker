use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    pub url: String,
    pub status: String,
    pub status_code: Option<u16>,
    pub response_time_ms: u64,
    pub timestamp: DateTime<Utc>,
    pub error: Option<String>,
}

impl HealthCheck {
    pub fn new_success(url: String, status_code: u16, response_time: Duration) -> Self {
        Self {
            url,
            status: "UP".to_string(),
            status_code: Some(status_code),
            response_time_ms: response_time.as_millis() as u64,
            timestamp: Utc::now(),
            error: None,
        }
    }

    pub fn new_failure(url: String, error: String, response_time: Duration) -> Self {
        Self {
            url,
            status: "DOWN".to_string(),
            status_code: None,
            response_time_ms: response_time.as_millis() as u64,
            timestamp: Utc::now(),
            error: Some(error),
        }
    }
}

pub struct HealthChecker {
    client: Client,
    #[allow(dead_code)]
    timeout: Duration,
}

impl HealthChecker {
    pub fn new(timeout: Duration) -> Self {
        let client = Client::builder()
            .timeout(timeout)
            .user_agent("rust-health-checker/0.1.0")
            .build()
            .expect("Failed to create HTTP client");

        Self { client, timeout }
    }

    pub async fn check_url(&self, url: &str) -> HealthCheck {
        use std::time::Instant;
        let start = Instant::now();

        match self.client.get(url).send().await {
            Ok(response) => {
                let status_code = response.status().as_u16();
                let response_time = start.elapsed();

                if response.status().is_success() {
                    HealthCheck::new_success(url.to_string(), status_code, response_time)
                } else {
                    HealthCheck::new_failure(
                        url.to_string(),
                        format!("HTTP {status_code}"),
                        response_time,
                    )
                }
            }
            Err(e) => {
                let response_time = start.elapsed();
                HealthCheck::new_failure(url.to_string(), e.to_string(), response_time)
            }
        }
    }

    pub async fn check_all_urls(&self, urls: &[String]) -> Vec<HealthCheck> {
        let futures = urls.iter().map(|url| self.check_url(url));
        futures::future::join_all(futures).await
    }
}

#[cfg(test)]
mod health_checker {
    use super::HealthChecker;
    use std::time::Duration;

    #[tokio::test]
    async fn test_successful_health_check() {
        let checker = HealthChecker::new(Duration::from_secs(10));
        let result = checker.check_url("https://httpbin.org/status/200").await;

        assert_eq!(result.status, "UP");
        assert_eq!(result.status_code, Some(200));
        assert!(result.error.is_none());
        assert!(result.response_time_ms > 0);
    }

    #[tokio::test]
    async fn test_failed_health_check() {
        let checker = HealthChecker::new(Duration::from_secs(5));
        let result = checker.check_url("https://httpbin.org/status/500").await;

        assert_eq!(result.status, "DOWN");
        assert_eq!(result.status_code, None);
        assert!(result.error.is_some());
    }

    #[tokio::test]
    async fn test_invalid_url() {
        let checker = HealthChecker::new(Duration::from_secs(5));
        let result = checker.check_url("not-a-valid-url").await;

        assert_eq!(result.status, "DOWN");
        assert!(result.error.is_some());
    }

    #[tokio::test]
    async fn test_multiple_urls() {
        let checker = HealthChecker::new(Duration::from_secs(10));
        let urls = vec![
            "https://httpbin.org/status/200".to_string(),
            "https://httpbin.org/status/404".to_string(),
        ];

        let results = checker.check_all_urls(&urls).await;

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].status, "UP");
        assert_eq!(results[1].status, "DOWN");
    }
}

#[cfg(test)]
mod tests {
    use super::HealthCheck;
    use std::time::Duration;

    #[test]
    fn test_health_check_creation() {
        let check = HealthCheck::new_success(
            "https://example.com".to_string(),
            200,
            Duration::from_millis(100),
        );

        assert_eq!(check.url, "https://example.com");
        assert_eq!(check.status, "UP");
        assert_eq!(check.response_time_ms, 100);
        assert!(check.error.is_none());
    }

    #[test]
    fn test_health_check_failure() {
        let check = HealthCheck::new_failure(
            "https://example.com".to_string(),
            "Connection failed".to_string(),
            Duration::from_millis(500),
        );

        assert_eq!(check.status, "DOWN");
        assert_eq!(check.status_code, None);
        assert_eq!(check.error, Some("Connection failed".to_string()));
    }
}
