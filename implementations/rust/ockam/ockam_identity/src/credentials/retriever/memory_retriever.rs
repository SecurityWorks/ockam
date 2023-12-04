use ockam_core::compat::boxed::Box;
use ockam_core::{async_trait, Result};
use ockam_node::Context;

use crate::models::CredentialAndPurposeKey;
use crate::{CredentialRetriever, Identifier};

/// Credentials retriever that retrieves a credential from memory
pub struct CredentialsMemoryRetriever {
    credential: CredentialAndPurposeKey,
}

impl CredentialsMemoryRetriever {
    /// Create a new CredentialsMemoryRetriever
    pub fn new(credential: CredentialAndPurposeKey) -> Self {
        Self { credential }
    }
}

#[async_trait]
impl CredentialRetriever for CredentialsMemoryRetriever {
    /// Retrieve a credential stored in memory
    async fn retrieve(
        &self,
        _ctx: &Context,
        _for_identity: &Identifier,
    ) -> Result<CredentialAndPurposeKey> {
        Ok(self.credential.clone())
    }
}
