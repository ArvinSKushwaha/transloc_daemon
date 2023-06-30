use serde::{Serialize, Deserialize};

use super::helper_types::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct EndpointResponse<Response> {
    pub api_latest_version: String,
    pub api_version: String,
    pub data: Response,
    pub expires_in: usize,
    pub generated_on: String,
    pub rate_limit: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct Agency {
    pub agency_id: AgencyId,
    pub bounding_box: GeographicArea,
    pub language: Language,
    pub long_name: String,
    pub name: String,
    pub phone: Option<String>,
    pub position: Location,
    pub short_name: String,
    pub timezone: String, // FIXME: AHHH
    pub url: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct Stop {
    pub agency_ids: Vec<AgencyId>,
    pub code: String,
    pub description: String,
    pub location: Location,
    pub location_type: LocationType,
    pub name: String,
    pub parent_station_id: Option<String>,
    pub routes: Vec<RouteId>,
    pub station_id: Option<String>,
    pub stop_id: StopId,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct Segment {
    pub agency_ids: Vec<AgencyId>,
    pub code: String,
    pub description: String,
    pub location: Location,
    pub location_type: LocationType,
    pub name: String,
    pub parent_station_id: Option<String>,
    pub routes: Vec<String>,
    pub station_id: Option<String>,
    pub stop_id: String,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct Vehicle {
    pub arrival_estimates: Vec<ArrivalEstimate>,
    pub call_name: String,
    pub description: Option<String>,
    pub heading: usize,
    pub last_updated_on: String,
    pub location: Location,
    pub passenger_load: Option<String>,
    pub route_id: RouteId,
    pub seating_capacity: Option<usize>,
    pub segment_id: Option<SegmentId>,
    pub speed: f32,
    pub standing_capacity: Option<usize>,
    pub tracking_status: String,
    pub vehicle_id: VehicleId,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct Route {
    pub agency_id: AgencyId,
    pub color: String,
    pub description: String,
    pub is_active: bool,
    pub is_hidden: bool,
    pub long_name: String,
    pub route_id: RouteId,
    pub segments: Vec<(SegmentId, Direction)>,
    pub short_name: String,
    pub stops: Vec<StopId>,
    pub text_color: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub url: String,
}
