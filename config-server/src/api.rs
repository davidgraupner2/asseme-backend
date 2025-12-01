use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiConfiguration {
    pub port: u16,
    pub behind_proxy: bool,
    pub request_timeout_secs: u64,
    pub agent_ping_interval: u64,
    pub agent_ping_timeout: u64,
    pub agent_jwt_secret: String,
    pub server_jwt_secret: String,
}

impl ApiConfiguration {
    pub fn default() -> Self {
        ApiConfiguration {
            port: 8000,
            behind_proxy: false,
            request_timeout_secs: 30,
            agent_ping_interval: 10,
            agent_ping_timeout: 5,
            agent_jwt_secret: ".AAuhSb@n7&aCW5{_Il3B&SQZKz$[_1cuES+P<n2kUD)-b0um?41Hg^|gN<&1|)O1#}EW,Y^ce5X3WV;,0xTLf".to_string(),
            server_jwt_secret: ",yTAs+WEZfbsfWLzGNFt-Nj<GQX7:sC.;W5/_gE=fGfubL/oLW^lN#X1YcwM?Ry&-a:U7{USG(Ez-zU{:vCmn^".to_string()
        }
    }
}

pub trait LoadApiConfiguration {
    fn load_config(&self) -> ApiConfiguration;
}
