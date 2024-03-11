use std::{future::ready, time::Instant};

use axum::{
    extract::{MatchedPath, Request},
    middleware::{self, Next},
    response::IntoResponse,
    routing::get,
    Router,
};
use http::{HeaderValue, Method};
use lazy_static::lazy_static;
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder};
use spotitube_core::errors::{SpotitubeError, SpotitubeResult};
use spotitube_infrastructure::service_register::ServiceRegister;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::endpoints::users_endpoints::UsersRouter;

lazy_static! {
    static ref HTTP_TIMEOUT: u64 = 30;
    static ref EXPONENTIAL_SECONDS: &'static [f64] =
        &[0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,];
}

pub struct SpotitubeApplicationController;

impl SpotitubeApplicationController {
    pub async fn serve(
        port: u32,
        cors_origin: &str,
        service_register: ServiceRegister,
    ) -> SpotitubeResult<()> {
        let recorder_handle = PrometheusBuilder::new()
            .set_buckets_for_metric(
                Matcher::Full(String::from("http_request_duration_seconds")),
                *EXPONENTIAL_SECONDS,
            )
            .and_then(|b| b.install_recorder())
            .map_err(|_| SpotitubeError::AppStartup)?;

        let router = Router::new()
            .nest("/api", UsersRouter::new_router(service_register))
            .route("/metrics", get(move || ready(recorder_handle.render())))
            .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
            .layer(
                CorsLayer::new()
                    .allow_origin(cors_origin.parse::<HeaderValue>().unwrap())
                    .allow_methods([Method::GET]),
            )
            .route_layer(middleware::from_fn(Self::track_matrics));

        let listener = TcpListener::bind(&format!("0.0.0.0:{}", port))
            .await
            .map_err(|_| SpotitubeError::AppStartup)?;
        axum::serve(listener, router.into_make_service())
            .await
            .map_err(|_| SpotitubeError::AppStartup)?;
        Ok(())
    }

    async fn track_matrics(request: Request, next: Next) -> impl IntoResponse {
        let path = match request.extensions().get::<MatchedPath>() {
            Some(matched_path) => matched_path.as_str().to_owned(),
            None => request.uri().path().to_owned(),
        };

        let start = Instant::now();
        let method = request.method().clone();
        let response = next.run(request).await;
        let latency = start.elapsed().as_secs_f64();
        let status = response.status().as_u16().to_string();

        let labels = [
            ("method", method.to_string()),
            ("path", path),
            ("status", status),
        ];

        metrics::counter!("http_requests_total", &labels).increment(1);
        metrics::histogram!("http_requests_duration_seconds", &labels).record(latency);

        response
    }
}
