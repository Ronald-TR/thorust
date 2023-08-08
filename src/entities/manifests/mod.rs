use std::{iter::Sum, ops::Add};

use serde::{Deserialize, Serialize};

use anyhow::Result;

use crate::traits::Manifest;

use self::{grpc::MGrpcFile, scripts::MScriptFile};

use super::{conversions::checks_depends_on, graph::TestNode};

pub mod grpc;
pub mod scripts;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseManifest {
    pub scripts: Option<MScriptFile>,
    pub grpc: Option<MGrpcFile>,
}

impl BaseManifest {
    pub fn new(scripts: Option<MScriptFile>, grpc: Option<MGrpcFile>) -> Self {
        Self { scripts, grpc }
    }
}

impl Manifest for BaseManifest {
    fn normalize(&mut self) -> Result<()> {
        if let Some(scripts) = &mut self.scripts {
            scripts.normalize()?;
        }
        if let Some(grpc) = &mut self.grpc {
            grpc.normalize()?;
        }
        let _ = self.as_test_nodes()?;
        Ok(())
    }
    fn as_test_nodes(&self) -> Result<Vec<TestNode>> {
        let mut nodes = Vec::new();
        if let Some(scripts) = &self.scripts {
            nodes.append(&mut scripts.as_test_nodes()?);
        }
        if let Some(grpc) = &self.grpc {
            nodes.append(&mut grpc.as_test_nodes()?);
        }
        checks_depends_on(&nodes)?;
        Ok(nodes)
    }
}

impl Add for BaseManifest {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let scripts = match &rhs.scripts {
            Some(other) => {
                let mut services = vec![];
                services.append(&mut other.services.to_vec());
                services.append(&mut self.scripts.map(|x| x.services).unwrap_or_default());
                Some(MScriptFile { services })
            }
            None => None,
        };
        let grpc = match &rhs.grpc {
            Some(other) => {
                let mut services = vec![];
                services.append(&mut other.services.to_vec());
                services.append(&mut self.grpc.map(|x| x.services).unwrap_or_default());
                Some(MGrpcFile { services })
            }
            None => None,
        };
        Self { scripts, grpc }
    }
}

impl Sum for BaseManifest {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(
            BaseManifest {
                scripts: None,
                grpc: None,
            },
            |acc, curr| acc + curr,
        )
    }
}
