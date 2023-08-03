use serde::{Deserialize, Serialize};

use self::{scripts::MScriptFile, grpc::MGrpcFile};

pub mod scripts;
pub mod grpc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseManifest {
    pub scripts: Option<MScriptFile>,
    pub grpc: Option<MGrpcFile>,
}