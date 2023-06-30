use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub(crate) struct GeographicArea(Location, Location);

#[derive(Debug, Clone, Deserialize, Serialize, Copy, PartialEq)]
pub(crate) struct Location {
    lat: f32,
    lng: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub(crate) enum LocationType {
    Stop,
}

#[derive(Debug, Clone, Deserialize, Serialize, Copy, PartialEq, Eq)]
pub(crate) enum Language {
    #[serde(rename = "en")]
    English,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub(crate) struct StopId(pub String);

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub(crate) struct RouteId(pub String);

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub(crate) struct SegmentId(pub String);

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub(crate) struct VehicleId(pub String);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct ArrivalEstimate {
    pub arrival_at: String,
    pub route_id: RouteId,
    pub stop_id: StopId,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Direction {
    Forward,
    Backward,
}

#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub(crate) struct AgencyId(#[serde(deserialize_with = "deserialize_string_or_number")] pub String);

pub(crate) fn deserialize_string_or_number<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNumber {
        String(String),
        Number(u64),
    }
    let deserialized = StringOrNumber::deserialize(deserializer)?;

    Ok(match deserialized {
        StringOrNumber::String(s) => s,
        StringOrNumber::Number(n) => n.to_string(),
    })
}
