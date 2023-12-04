use crate::nodes::service::{NodeManagerCredentialRetrieverOptions, NodeManagerTrustOptions};
use crate::{multiaddr_to_route, CliState, DefaultAddress};
use ockam::identity::RemoteCredentialRetrieverInfo;
use ockam_core::Result;
use ockam_transport_tcp::TcpTransport;

impl CliState {
    pub async fn retrieve_trust_options(
        &self,
        project_name: &Option<String>,
        tcp: &TcpTransport,
    ) -> Result<NodeManagerTrustOptions> {
        let project = match project_name {
            Some(project_name) => self.get_project_by_name(project_name).await.ok(),
            None => self.get_default_project().await.ok(),
        };

        let project = match project {
            Some(project) => project,
            None => {
                return Ok(NodeManagerTrustOptions::new(
                    NodeManagerCredentialRetrieverOptions::None,
                    None,
                ))
            }
        };

        let authority_identifier = project.authority_identifier().await?;
        let info = RemoteCredentialRetrieverInfo::new(
            authority_identifier.clone(),
            multiaddr_to_route(&project.authority_access_route()?, tcp)
                .await
                .unwrap()
                .route,
            DefaultAddress::CREDENTIAL_ISSUER.into(),
        );

        let trust_options = NodeManagerTrustOptions::new(
            NodeManagerCredentialRetrieverOptions::Remote(info),
            Some(authority_identifier),
        );

        Ok(trust_options)
    }
}
