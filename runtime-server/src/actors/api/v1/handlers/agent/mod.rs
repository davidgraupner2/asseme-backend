pub(crate) mod protocol;
pub(crate) mod types;

use crate::actors::api::{
    state::{ApiState, V1ApiState},
    v1::{
        api_response::{ApiError, ApiResponse},
        handlers::agent::{
            protocol::{Inbound, Outbound},
            types::{AgentEntry, AgentInfo, AgentRegistry},
        },
        jwt::{generate_jwt, JwtType},
    },
};
use axum::{
    extract::{
        ws::{Message, WebSocket},
        Query, State, WebSocketUpgrade,
    },
    response::IntoResponse,
    Extension,
};
use futures_util::{SinkExt, StreamExt};
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot, Mutex};
use tracing::{error, info, instrument};
use types::WSConnect;
use uuid::Uuid;

#[derive(Serialize)]
pub struct AgentToken {
    token: String,
}

#[instrument(name = "Agent Token Generator", level = "trace")]
pub async fn get_agent_token_handler(
    State(state): State<Arc<ApiState>>,
    Extension(v1_state): Extension<Arc<V1ApiState>>,
) -> Result<ApiResponse<AgentToken>, ApiError> {
    // #TODO - need to ensure the tenant is selected from the auth bearer token
    let tenant = "streamline_partners";

    match generate_jwt(tenant, &state.agent_jwt_secret, JwtType::Agent) {
        Ok(jwt) => {
            let token = AgentToken { token: jwt };
            Ok(ApiResponse::ok(token))
        }
        Err(error) => {
            error!(error=%error,"Failed to generate Agent JWT");
            Err(ApiError::Internal(format!(
                "Failed to generate Agent JWT - {}",
                error.to_string()
            )))
        }
    }
}

#[instrument(name = "Agent Connection Handler", level = "trace")]
pub async fn agent_connection_handler(
    ws: WebSocketUpgrade,
    Query(params): Query<WSConnect>,
    State(state): State<Arc<ApiState>>,
    Extension(v1_state): Extension<Arc<V1ApiState>>,
) -> impl IntoResponse {
    println!("PARAMS: {:?}", params);

    let id = params.id;
    let groups = params
        .groups
        .into_iter()
        .flat_map(|s| {
            s.split(',')
                .map(|p| p.trim().to_string())
                .filter(|p| !p.is_empty())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let token = params.token;

    // capture owned values into the on_upgrade closure
    ws.on_upgrade(move |socket| handle_socket(socket, id, token, groups, state, v1_state))
}

#[instrument(name = "Hande Agent Socket Connection", level = "trace")]
async fn handle_socket(
    socket: WebSocket,
    agent_id: String,
    token: String,
    groups: Vec<String>,
    state: Arc<ApiState>,
    v1_state: Arc<V1ApiState>,
) {
    // split socket into sink and stream
    let (mut sender, mut receiver) = socket.split();

    // outbound channel + writer task
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();
    let write_task = tokio::spawn(async move {
        while let Some(text) = rx.recv().await {
            if sender.send(Message::Text(text.into())).await.is_err() {
                break;
            }
        }
    });

    // create AgentEntry and insert into registry immediately
    let info = Arc::new(AgentInfo {
        id: agent_id.clone(),
        groups: groups.clone(),
        last_seen: Mutex::new(chrono::Utc::now()),
        pending_pong: Mutex::new(None),
        token: token,
    });

    let entry = AgentEntry {
        info: info.clone(),
        tx: tx.clone(),
    };

    v1_state
        .agent_registry
        .insert(agent_id.clone(), entry.clone());
    info!(%agent_id, "agent connected (from querystring)");

    // spawn heartbeat monitor
    tokio::spawn(start_heartbeat(
        agent_id.clone(),
        entry.clone(),
        v1_state.agent_registry.clone(),
        state.agent_ping_interval,
        state.agent_ping_timeout,
    ));

    // read loop: handle Pong / Ack / Disconnect
    while let Some(Ok(msg)) = receiver.next().await {
        match msg {
            Message::Text(t) => {
                if let Ok(inbound) = serde_json::from_str::<Inbound>(&t) {
                    match inbound {
                        Inbound::Pong { nonce: _ } => {
                            // resolve pending pong oneshot if present
                            if let Some(entry) = v1_state.agent_registry.get(&agent_id) {
                                let info = &entry.info;
                                let mut last = info.last_seen.lock().await;
                                *last = chrono::Utc::now();
                                if let Some(sender) = info.pending_pong.lock().await.take() {
                                    let _ = sender.send(());
                                }
                            }
                        }
                        Inbound::Disconnect { reason } => {
                            info!(agent = %agent_id, ?reason, "agent requested disconnect");
                            v1_state.agent_registry.remove(&agent_id);
                            let _ = tx.send(
                                serde_json::to_string(&Outbound::Disconnect { reason }).unwrap(),
                            );
                            break;
                        }
                        Inbound::Ack { command_id } => {
                            info!(agent = %agent_id, %command_id, "ack received");
                        }
                    }
                }
            }
            Message::Close(_) => {
                info!(agent = %agent_id, "socket closed by client");
                v1_state.agent_registry.remove(&agent_id);
                break;
            }
            _ => {}
        }
    }

    // ensure writer finishes
    let _ = write_task.await;
}

// ---------- Heartbeat monitor (same as earlier) ----------
#[instrument(name = "Agent Heartbeat", level = "trace")]
async fn start_heartbeat(
    agent_id: String,
    entry: AgentEntry,
    registry: AgentRegistry,
    ping_interval_seconds: u64,
    ping_timeout_seconds: u64,
) {
    loop {
        let nonce = Uuid::new_v4().to_string();
        let ping = Outbound::Ping {
            nonce: nonce.clone(),
        };
        let text = serde_json::to_string(&ping).unwrap();

        let (tx, rx) = oneshot::channel::<()>();
        {
            let mut pending = entry.info.pending_pong.lock().await;
            *pending = Some(tx);
        }

        if entry.tx.send(text).is_err() {
            registry.remove(&agent_id);
            break;
        }

        println!("Interval: {}", ping_interval_seconds);
        println!("Timeout: {}", ping_timeout_seconds);

        match tokio::time::timeout(std::time::Duration::from_secs(ping_timeout_seconds), rx).await {
            Ok(Ok(_)) => {
                let mut last = entry.info.last_seen.lock().await;
                *last = chrono::Utc::now();
            }
            _ => {
                info!(agent = %agent_id, "ping timeout - disconnecting agent");
                registry.remove(&agent_id);
                let _ = entry.tx.send(
                    serde_json::to_string(&Outbound::Disconnect {
                        reason: Some("ping timeout".into()),
                    })
                    .unwrap(),
                );
                break;
            }
        }

        tokio::time::sleep(std::time::Duration::from_secs(ping_interval_seconds)).await;
    }
}

// ---------- Helpers: direct/group/broadcast sends ----------
#[instrument(name = "Send to Agent", level = "trace")]
async fn send_to_agent(registry: &AgentRegistry, id: &str, msg: Outbound) -> Result<(), ()> {
    if let Some(entry) = registry.get(id) {
        let s = serde_json::to_string(&msg).unwrap();
        entry.tx.send(s).map_err(|_| ())?;
        Ok(())
    } else {
        Err(())
    }
}

#[instrument(name = "Send to Agent Group", level = "trace")]
async fn send_to_group(registry: &AgentRegistry, group: &str, msg: Outbound) {
    for r in registry.iter() {
        if r.value().info.groups.contains(&group.to_string()) {
            let _ = r.value().tx.send(serde_json::to_string(&msg).unwrap());
        }
    }
}

#[instrument(name = "Broadcast to Agents", level = "trace")]
async fn broadcast(registry: &AgentRegistry, msg: Outbound) {
    for r in registry.iter() {
        let _ = r.value().tx.send(serde_json::to_string(&msg).unwrap());
    }
}
