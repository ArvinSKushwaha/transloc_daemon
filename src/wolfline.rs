use std::collections::HashMap;

use thiserror::Error;

use crate::api::{
    helper_types::{AgencyId, ArrivalEstimate, LocationType, RouteId, StopId, VehicleId},
    request::EndpointRequest,
    response::{Agency, EndpointResponse, Route, Stop, Vehicle},
    TranslocApiManager,
};

pub(crate) struct WolfLine {
    pub(crate) tlm: TranslocApiManager,
    pub(crate) agency: Option<AgencyId>,
    pub(crate) route: Option<RouteId>,
    pub(crate) vehicle: Option<VehicleId>,
    pub(crate) stop: Option<StopId>,
}

#[derive(Error, Debug)]
pub enum WolfLineError {
    #[error("Reqwest Error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Serde Error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("No NCSU Agency found")]
    NoAgencyFound,
    #[error("No NCSU Route found")]
    NoRouteFound,
    #[error("No NCSU Stop found")]
    NoStopFound,
    #[error("Invalid TransLoc API Response")]
    InvalidResponse,
}

impl WolfLine {
    pub(crate) fn new(api_key: &'static str) -> anyhow::Result<Self> {
        Ok(Self {
            tlm: TranslocApiManager::new(api_key)?,
            agency: None,
            route: None,
            vehicle: None,
            stop: None,
        })
    }

    pub(crate) async fn find_ncsu(&mut self) -> Result<(), WolfLineError> {
        let req = self
            .tlm
            .request(EndpointRequest::Agencies {
                agencies: None,
                geo_area: None,
            })
            .await?;

        let agency_resp: EndpointResponse<Vec<Agency>> = serde_json::from_str(&req.text().await?)?;
        let agency = agency_resp
            .data
            .iter()
            .filter(|i| i.name == "ncsu")
            .next()
            .unwrap();

        self.agency = Some(agency.agency_id.clone());

        Ok(())
    }

    pub(crate) async fn get_routes(&self) -> Result<Vec<Route>, WolfLineError> {
        let Some(agency_id) = self.agency.as_ref() else { return Err(WolfLineError::NoAgencyFound) };

        let req = self
            .tlm
            .request(EndpointRequest::Routes {
                agencies: self.agency.clone().into_iter().collect(),
                geo_area: None,
            })
            .await?;

        let data = &req.text().await?;
        let mut agency_resp: EndpointResponse<HashMap<AgencyId, Vec<Route>>> =
            serde_json::from_str(data)?;
        agency_resp
            .data
            .remove(&agency_id)
            .ok_or(WolfLineError::InvalidResponse)
    }

    pub(crate) async fn find_route(&mut self, route: &str) -> Result<(), WolfLineError> {
        let routes = self.get_routes().await?;
        self.route = fuzzy_match::fuzzy_match(
            route,
            routes
                .iter()
                .map(|r| (r.long_name.as_str(), r.route_id.clone())),
        );

        Ok(())
    }

    pub(crate) async fn get_vehicles(&self) -> Result<Vec<Vehicle>, WolfLineError> {
        let Some(agency_id) = self.agency.as_ref() else { return Err(WolfLineError::NoAgencyFound) };
        let Some(route_id) = self.route.as_ref() else { return Err(WolfLineError::NoRouteFound ) };

        let req = self
            .tlm
            .request(EndpointRequest::Vehicles {
                agencies: Some(self.agency.clone().into_iter().collect()),
                geo_area: None,
            })
            .await?;

        let data = &req.text().await?;
        let mut vehicle_resp: EndpointResponse<HashMap<AgencyId, Vec<Vehicle>>> =
            serde_json::from_str(data)?;
        let vehicles = vehicle_resp
            .data
            .remove(&agency_id)
            .ok_or(WolfLineError::InvalidResponse)?;

        Ok(vehicles
            .iter()
            .filter(|v| &v.route_id == route_id)
            .cloned()
            .collect())
    }

    pub(crate) async fn get_stops(&mut self) -> Result<Vec<Stop>, WolfLineError> {
        let Some(ref agency_id) = self.agency else { return Err(WolfLineError::NoAgencyFound) };
        let Some(ref route_id) = self.route else { return Err(WolfLineError::NoRouteFound ) };

        let req = self
            .tlm
            .request(EndpointRequest::Stops {
                agencies: self.agency.clone().into_iter().collect(),
                geo_area: None,
            })
            .await?;

        let data = &req.text().await?;
        let agency_resp: EndpointResponse<Vec<Stop>> = serde_json::from_str(data)?;

        Ok(agency_resp
            .data
            .iter()
            .filter(|s| s.agency_ids.contains(&agency_id) && s.routes.contains(&route_id))
            .cloned()
            .collect())
    }

    pub(crate) async fn find_stop(&mut self, stop: &str) -> Result<(), WolfLineError> {
        let stops = self.get_stops().await?;
        self.stop = fuzzy_match::fuzzy_match(
            stop,
            stops.iter().filter_map(|f| {
                if f.location_type == LocationType::Stop {
                    Some((f.name.as_str(), f.stop_id.clone()))
                } else {
                    None
                }
            }),
        );
        Ok(())
    }

    pub(crate) async fn get_arrival_estimates(
        &self,
    ) -> Result<Vec<ArrivalEstimate>, WolfLineError> {
        let Some(ref stop_id) = self.stop else { return Err(WolfLineError::NoStopFound) };

        let vehicles = self.get_vehicles().await?;
        let estimates = vehicles
            .into_iter()
            .map(|v| {
                v.arrival_estimates
                    .into_iter()
                    .filter(|a| &a.stop_id == stop_id)
            })
            .flatten()
            .collect();

        Ok(estimates)
    }

    pub(crate) async fn get_time_remaining(&self) -> Result<Vec<chrono::Duration>, WolfLineError> {
        let estimates = self.get_arrival_estimates().await?;
        println!("{:?}", estimates);
        let durations = estimates
            .into_iter()
            .filter_map(
                |f| match chrono::DateTime::parse_from_rfc3339(&f.arrival_at) {
                    Ok(parsed) => Some(parsed),
                    Err(err) => {
                        dbg!(err);
                        None
                    }
                },
            )
            .map(|dt| dt.signed_duration_since(chrono::Local::now()))
            .collect();

        Ok(durations)
    }
}
