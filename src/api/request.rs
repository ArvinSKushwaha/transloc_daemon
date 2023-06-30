use crate::API_URL;

use super::helper_types::{AgencyId, Location, StopId};

#[derive(Clone, Debug)]
pub(crate) enum EndpointRequest {
    Vehicles {
        agencies: Option<Vec<AgencyId>>,
        geo_area: Option<Location>,
    },
    Stops {
        agencies: Vec<AgencyId>,
        geo_area: Option<Location>,
    },
    Segments {
        agencies: Vec<AgencyId>,
        geo_area: Option<Location>,
        routes: Option<Vec<AgencyId>>,
    },
    Routes {
        agencies: Vec<AgencyId>,
        geo_area: Option<Location>,
    },
    ArrivalEstimates {
        agencies: Vec<AgencyId>,
        stops: Vec<StopId>,
        geo_area: Option<Location>,
    },
    Agencies {
        agencies: Option<Vec<AgencyId>>,
        geo_area: Option<Location>,
    },
}

impl EndpointRequest {
    pub(crate) fn to_url(&self) -> String {
        let route = match self {
            EndpointRequest::Vehicles { .. } => "vehicles.json".to_string(),
            EndpointRequest::Stops { .. } => "stops.json".to_string(),
            EndpointRequest::Segments { .. } => "segments.json".to_string(),
            EndpointRequest::Routes { .. } => "routes.json".to_string(),
            EndpointRequest::ArrivalEstimates { .. } => "arrival-estimates.json".to_string(),
            EndpointRequest::Agencies { .. } => "agencies.json".to_string(),
        };

        format!("https://{}/{}", API_URL, route)
    }

    pub(crate) fn fill_query(&self, query: &mut Vec<(String, String)>) {
        match self {
            EndpointRequest::Vehicles { agencies, geo_area } => {
                if let Some(agencies) = agencies {
                    let agencies = agencies.iter().cloned().map(|a| a.0).collect::<Vec<_>>();
                    query.push(("agencies".to_string(), agencies.join(",")));
                }
            }
            EndpointRequest::Stops { agencies, geo_area } => {
                let agencies = agencies.iter().cloned().map(|a| a.0).collect::<Vec<_>>();
                query.push(("agencies".to_string(), agencies.join(",")));
            }
            EndpointRequest::Segments {
                agencies,
                geo_area,
                routes,
            } => {
                let agencies = agencies.iter().cloned().map(|a| a.0).collect::<Vec<_>>();
                query.push(("agencies".to_string(), agencies.join(",")));

                if let Some(routes) = routes {
                    let routes = routes.iter().cloned().map(|a| a.0).collect::<Vec<_>>();
                    query.push(("routes".to_string(), routes.join(",")));
                }
            }
            EndpointRequest::Routes { agencies, geo_area } => {
                let agencies = agencies.iter().cloned().map(|a| a.0).collect::<Vec<_>>();
                query.push(("agencies".to_string(), agencies.join(",")));
            }
            EndpointRequest::ArrivalEstimates {
                agencies,
                stops,
                geo_area,
            } => {
                let agencies = agencies.iter().cloned().map(|a| a.0).collect::<Vec<_>>();
                query.push(("agencies".to_string(), agencies.join(",")));
                let stops = stops.iter().cloned().map(|a| a.0).collect::<Vec<_>>();
                query.push(("stops".to_string(), stops.join(",")));
            }
            EndpointRequest::Agencies { agencies, geo_area } => {
                if let Some(agencies) = agencies {
                    let agencies = agencies.iter().cloned().map(|a| a.0).collect::<Vec<_>>();
                    query.push(("agencies".to_string(), agencies.join(",")));
                }
            }
        }
    }
}
