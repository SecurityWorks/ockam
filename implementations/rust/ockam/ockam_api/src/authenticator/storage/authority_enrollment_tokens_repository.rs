use crate::authenticator::one_time_code::OneTimeCode;
use crate::authenticator::Token;
use ockam::identity::TimestampInSeconds;
use ockam_core::async_trait;
use ockam_core::compat::boxed::Box;
use ockam_core::Result;

/// This repository stores enrollment tokens on the Authority node
#[async_trait]
pub trait AuthorityEnrollmentTokensRepository: Send + Sync + 'static {
    /// Use previously issued token
    async fn use_token(
        &self,
        one_time_code: OneTimeCode,
        now: TimestampInSeconds,
    ) -> Result<Option<Token>>;

    /// Issue a new enrolment token
    async fn issue_token(&self, token: Token) -> Result<()>;
}
