use std::env;

use tokio_stream::StreamExt;
use tonic::transport::Channel;

use macro_traffic_sim::pb;
use macro_traffic_sim::pb::macro_service_client::MacroServiceClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Server address (override with MACRO_SIM_ADDR)
    let raw = env::var("MACRO_SIM_ADDR").unwrap_or_else(|_| "127.0.0.1:50052".to_string());
    let addr = if raw.starts_with("http://") || raw.starts_with("https://") {
        raw
    } else {
        format!("http://{raw}")
    };

    // Connect
    let channel = Channel::from_shared(addr.clone())?
        .connect()
        .await?;
    let mut client = MacroServiceClient::new(channel);

    // ---- STEP 1: CREATE SESSION ----
    let resp = client.new_session(pb::NewSessionRequest {}).await?.into_inner();
    let sid = resp
        .session_id
        .as_ref()
        .map(|x| x.value.clone())
        .ok_or("server returned empty session id")?;
    println!("Session created: {}", sid);

    let session_id = || Some(pb::UuiDv4 { value: sid.clone() });

    // ---- STEP 2: PUSH NETWORK ----
    // Simple 5-node, 4-link network:
    //   Zone1(N1) --L1--> N2 --L2--> N3 --L3--> N4 --L4--> N5(Zone2)
    let nodes = vec![
        pb::Node { id: 1, longitude: 37.60, latitude: 55.75, zone_id: 1, macro_node_id: -1, macro_link_id: -1 },
        pb::Node { id: 2, longitude: 37.61, latitude: 55.75, zone_id: -1, macro_node_id: -1, macro_link_id: -1 },
        pb::Node { id: 3, longitude: 37.62, latitude: 55.75, zone_id: -1, macro_node_id: -1, macro_link_id: -1 },
        pb::Node { id: 4, longitude: 37.63, latitude: 55.75, zone_id: -1, macro_node_id: -1, macro_link_id: -1 },
        pb::Node { id: 5, longitude: 37.64, latitude: 55.75, zone_id: 2, macro_node_id: -1, macro_link_id: -1 },
    ];

    let links = vec![
        pb::Link { id: 1, source_node_id: 1, target_node_id: 2, length_meters: 500.0, free_speed: 60.0, capacity: 1800.0, lanes: 2, link_type: 3, is_connection: false, macro_link_id: -1, movement_id: -1 },
        pb::Link { id: 2, source_node_id: 2, target_node_id: 3, length_meters: 500.0, free_speed: 60.0, capacity: 1800.0, lanes: 2, link_type: 3, is_connection: false, macro_link_id: -1, movement_id: -1 },
        pb::Link { id: 3, source_node_id: 3, target_node_id: 4, length_meters: 500.0, free_speed: 60.0, capacity: 1800.0, lanes: 2, link_type: 3, is_connection: false, macro_link_id: -1, movement_id: -1 },
        pb::Link { id: 4, source_node_id: 4, target_node_id: 5, length_meters: 500.0, free_speed: 60.0, capacity: 1800.0, lanes: 2, link_type: 3, is_connection: false, macro_link_id: -1, movement_id: -1 },
    ];

    let network_stream = tokio_stream::once(pb::NetworkChunk {
        session_id: session_id(),
        nodes,
        links,
    });
    let mut resp_stream = client.push_network(network_stream).await?.into_inner();
    while let Some(resp) = resp_stream.next().await {
        let r = resp?;
        println!("Network: {} nodes, {} links ({})", r.nodes_received, r.links_received, r.text);
    }

    // ---- STEP 3: PUSH ZONES ----
    let zones = vec![
        pb::Zone { id: 1, name: "Zone A".to_string(), population: 10000.0, employment: 3000.0, households: 4000.0, avg_income: 50000.0, area_sq_km: 2.5 },
        pb::Zone { id: 2, name: "Zone B".to_string(), population: 8000.0, employment: 6000.0, households: 3000.0, avg_income: 45000.0, area_sq_km: 3.0 },
    ];

    let zone_stream = tokio_stream::once(pb::ZoneChunk {
        session_id: session_id(),
        zones,
    });
    let mut resp_stream = client.push_zones(zone_stream).await?.into_inner();
    while let Some(resp) = resp_stream.next().await {
        let r = resp?;
        println!("Zones: {} received ({})", r.zones_received, r.text);
    }

    // ---- STEP 4: SET CONFIG ----
    let config_resp = client.set_model_config(pb::ModelConfigRequest {
        session_id: session_id(),
        assignment_method: 0, // Frank-Wolfe
        bpr: Some(pb::BprConfig { alpha: 0.15, beta: 4.0 }),
        assignment: Some(pb::AssignmentConvergence { max_iterations: 50, convergence_gap: 0.001 }),
        furness: Some(pb::FurnessConfig { max_iterations: 100, tolerance: 0.001 }),
        feedback_iterations: 2,
        gp_step_scale: 0.1,
        impedance: Some(pb::ImpedanceFunction { r#type: 0, alpha: 0.0, beta: 0.1 }),
        trip_generation: Some(pb::TripGenerationConfig {
            method: 0, // Regression
            production_coeffs: Some(pb::RegressionCoeffs { intercept: 0.0, pop_coeff: 0.5, emp_coeff: 0.1, hh_coeff: 0.0, income_coeff: 0.0 }),
            attraction_coeffs: Some(pb::RegressionCoeffs { intercept: 0.0, pop_coeff: 0.1, emp_coeff: 0.8, hh_coeff: 0.0, income_coeff: 0.0 }),
            attraction_rate_per_employee: 0.0,
            hh_size_thresholds: vec![],
            income_thresholds: vec![],
            production_rates: Default::default(),
        }),
        mode_utilities: vec![
            pb::ModeUtility { mode: "auto".to_string(), asc: 0.0, coeff_time: -0.03, coeff_distance: 0.0, coeff_cost: -0.05 },
            pb::ModeUtility { mode: "bike".to_string(), asc: -1.5, coeff_time: -0.04, coeff_distance: -0.1, coeff_cost: 0.0 },
            pb::ModeUtility { mode: "walk".to_string(), asc: -2.0, coeff_time: -0.05, coeff_distance: -0.2, coeff_cost: 0.0 },
        ],
        time_periods: vec![],
    }).await?.into_inner();
    println!("Config: {} ({})", config_resp.code, config_resp.text);

    // ---- STEP 5: RUN PIPELINE ----
    println!("\nRunning pipeline...");
    let mut progress_stream = client.run_pipeline(pb::RunPipelineRequest {
        session_id: session_id(),
    }).await?.into_inner();
    while let Some(progress) = progress_stream.next().await {
        let p = progress?;
        if p.is_failed {
            println!("FAILED: {}", p.error_message);
            return Err(p.error_message.into());
        }
        println!(
            "  [period {}/{}] phase={}, feedback={}/{}, iter={}/{}, gap={:.6}{}",
            p.period_current, p.period_total,
            p.phase,
            p.feedback_current, p.feedback_total,
            p.assignment_iteration, p.assignment_max_iterations,
            p.relative_gap,
            if p.is_completed { " DONE" } else { "" },
        );
    }

    // ---- STEP 6: GET RESULTS ----
    println!("\nLink volumes:");
    let mut vol_stream = client.get_link_volumes(pb::LinkVolumesRequest {
        session_id: session_id(),
        period_index: 0,
    }).await?.into_inner();
    while let Some(chunk) = vol_stream.next().await {
        let c = chunk?;
        for v in &c.volumes {
            println!("  link {}: volume={:.2}, time={:.6}", v.link_id, v.volume, v.travel_time);
        }
    }

    // Assignment info
    let info = client.get_assignment_info(pb::AssignmentInfoRequest {
        session_id: session_id(),
        period_index: 0,
    }).await?.into_inner();
    if let Some(ai) = &info.info {
        println!("\nAssignment: {} iterations, gap={:.8}, converged={}", ai.iterations, ai.relative_gap, ai.converged);
    }

    // ---- STEP 7: CLEANUP ----
    let del = client.delete_session(pb::SessionId { value: session_id() }).await?.into_inner();
    println!("\nSession deleted: {} ({})", del.code, del.text);

    Ok(())
}
