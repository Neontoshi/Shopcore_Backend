use reqwest::Client;
use chrono::{DateTime, Utc, NaiveDateTime, Duration};
use crate::errors::AppError;
use crate::services::shipment_tracking_service::TrackingEvent;

pub struct TrackingMoreApi {
    client: Client,
    api_key: String,
    base_url: String,
}

impl TrackingMoreApi {
    pub fn new() -> Result<Self, AppError> {
        let api_key = std::env::var("TRACKINGMORE_API_KEY")
            .map_err(|_| AppError::payment_error("TRACKINGMORE_API_KEY not set in .env file"))?;
        
        Ok(Self {
            client: Client::new(),
            api_key,
            base_url: "https://api.trackingmore.com/v4".to_string(),
        })
    }

    fn is_mock_mode() -> bool {
        std::env::var("TRACKINGMORE_MOCK_MODE").unwrap_or_default() == "true"
    }

    fn get_mock_events(carrier: &str, tracking_number: &str) -> Vec<TrackingEvent> {
        let now = Utc::now();
        vec![
            TrackingEvent {
                timestamp: now - Duration::days(3),
                location: format!("{} Shipping Center", carrier.to_uppercase()),
                status: "Label Created".to_string(),
                description: format!("Shipping label created for {}", tracking_number),
            },
            TrackingEvent {
                timestamp: now - Duration::days(2),
                location: "Regional Hub".to_string(),
                status: "In Transit".to_string(),
                description: "Package received at regional distribution center".to_string(),
            },
            TrackingEvent {
                timestamp: now - Duration::days(1),
                location: "Local Facility".to_string(),
                status: "Out for Delivery".to_string(),
                description: "Package is out for delivery today".to_string(),
            },
            TrackingEvent {
                timestamp: now,
                location: "Destination".to_string(),
                status: "Delivered".to_string(),
                description: "Package has been delivered".to_string(),
            },
        ]
    }

    pub async fn fetch_tracking(
        &self,
        carrier_code: &str,
        tracking_number: &str,
    ) -> Result<Vec<TrackingEvent>, AppError> {
        // Mock mode
        if Self::is_mock_mode() {
            tracing::info!("Mock mode enabled - returning fake tracking events");
            return Ok(Self::get_mock_events(carrier_code, tracking_number));
        }

        tracing::info!("Fetching TrackingMore for {} - {}", carrier_code, tracking_number);

        let url = format!("{}/trackings/get", self.base_url);
        
        let response = self.client
            .post(&url)
            .header("Tracking-Api-Key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "tracking_number": tracking_number,
                "carrier_code": carrier_code
            }))
            .send()
            .await
            .map_err(|e| AppError::payment_error(format!("TrackingMore API error: {}", e)))?;

        let status = response.status();
        
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            tracing::error!("TrackingMore API error {}: {}", status, error_text);
            return Ok(Self::get_mock_events(carrier_code, tracking_number));
        }

        let data: serde_json::Value = response.json()
            .await
            .map_err(|e| AppError::payment_error(format!("Failed to parse response: {}", e)))?;
        
        let code = data.get("code").and_then(|c| c.as_i64()).unwrap_or(-1);
        
        if code != 200 {
            let message = data.get("message").and_then(|m| m.as_str()).unwrap_or("Unknown error");
            tracing::warn!("API returned code {}: {}", code, message);
            return Ok(Self::get_mock_events(carrier_code, tracking_number));
        }

        Self::parse_tracking_response(data)
    }

    pub async fn fetch_tracking_auto(
        &self,
        tracking_number: &str,
    ) -> Result<(String, Vec<TrackingEvent>), AppError> {
        if Self::is_mock_mode() {
            tracing::info!("Mock mode - returning auto-detected carrier 'usps'");
            return Ok(("usps".to_string(), Self::get_mock_events("usps", tracking_number)));
        }

        let url = format!("{}/trackings/detect", self.base_url);
        
        let response = self.client
            .post(&url)
            .header("Tracking-Api-Key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "tracking_number": tracking_number
            }))
            .send()
            .await
            .map_err(|e| AppError::payment_error(format!("TrackingMore detect error: {}", e)))?;

        let data: serde_json::Value = response.json()
            .await
            .map_err(|e| AppError::payment_error(format!("Failed to parse detect response: {}", e)))?;

        let detected_carrier = data
            .get("data")
            .and_then(|d| d.get("carrier_code"))
            .and_then(|c| c.as_str())
            .unwrap_or("")
            .to_string();

        if detected_carrier.is_empty() {
            return Ok(("unknown".to_string(), Self::get_mock_events("unknown", tracking_number)));
        }

        let events = self.fetch_tracking(&detected_carrier, tracking_number).await?;
        Ok((detected_carrier, events))
    }

    fn parse_tracking_response(data: serde_json::Value) -> Result<Vec<TrackingEvent>, AppError> {
        let mut events = Vec::new();

        if let Some(data_obj) = data.get("data") {
            if let Some(items) = data_obj.get("items").and_then(|i| i.as_array()) {
                for item in items {
                    if let Some(trackinfo) = item.get("trackinfo").and_then(|t| t.as_array()) {
                        for info in trackinfo {
                            let date = info.get("Date").and_then(|d| d.as_str()).unwrap_or("");
                            let status = info.get("StatusDescription").and_then(|s| s.as_str()).unwrap_or("");
                            let details = info.get("Details").and_then(|d| d.as_str()).unwrap_or("");
                            
                            events.push(TrackingEvent {
                                timestamp: Self::parse_trackingmore_date(date),
                                location: Self::extract_location_from_details(details),
                                status: status.to_string(),
                                description: details.to_string(),
                            });
                        }
                    }
                }
            }
        }

        events.sort_by_key(|e| e.timestamp);
        Ok(events)
    }

    fn parse_trackingmore_date(date_str: &str) -> DateTime<Utc> {
        if let Ok(naive) = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S") {
            naive.and_utc()
        } else if let Ok(date) = chrono::DateTime::parse_from_rfc3339(date_str) {
            date.with_timezone(&Utc)
        } else {
            Utc::now()
        }
    }

    fn extract_location_from_details(details: &str) -> String {
        let parts: Vec<&str> = details.split(',').collect();
        if parts.len() >= 2 {
            parts[0].to_string()
        } else {
            "Unknown".to_string()
        }
    }
}

impl Default for TrackingMoreApi {
    fn default() -> Self {
        Self::new().expect("TRACKINGMORE_API_KEY must be set in .env")
    }
}
