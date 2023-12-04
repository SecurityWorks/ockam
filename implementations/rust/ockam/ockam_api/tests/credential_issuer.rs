use minicbor::bytes::ByteSlice;
use ockam::identity::models::CredentialAndPurposeKey;
use ockam::identity::utils::now;
use ockam::identity::{identities, AttributesEntry};
use ockam::identity::{
    Identities, SecureChannelListenerOptions, SecureChannelOptions, SecureChannels,
};
use ockam::route;
use ockam_api::authenticator::credentials_issuer::CredentialsIssuer;
use ockam_api::authenticator::{
    AuthorityMembersRepository, AuthorityMembersSqlxDatabase, PreTrustedIdentities,
};
use ockam_core::api::Request;
use ockam_core::compat::collections::{BTreeMap, HashMap};
use ockam_core::{Address, Result};
use ockam_node::api::Client;
use ockam_node::Context;

#[ockam_macros::test]
async fn credential(ctx: &mut Context) -> Result<()> {
    let api_worker_addr = Address::random_local();
    let auth_worker_addr = Address::random_local();

    // create 2 identities to populate the trusted identities
    let identities = identities().await?;
    let auth_identifier = identities.identities_creation().create_identity().await?;
    let member_identifier = identities.identities_creation().create_identity().await?;
    let member_identity = identities.get_identity(&member_identifier).await?;

    let now = now().unwrap();

    let pre_trusted = HashMap::from([(
        member_identifier.clone(),
        AttributesEntry::new(
            BTreeMap::from([(b"attr".to_vec(), b"value".to_vec())]),
            now,
            None,
            None,
        ),
    )]);

    let pre_trusted_identities = PreTrustedIdentities::new_from_hashmap(pre_trusted);
    let members = AuthorityMembersSqlxDatabase::create().await?;
    members
        .bootstrap_pre_trusted_members(&pre_trusted_identities)
        .await?;

    // Now recreate the identities services with the previous vault
    // (so that the authority can verify its signature)
    // and the repository containing the trusted identities
    let identities = Identities::builder()
        .await?
        .with_change_history_repository(identities.change_history_repository())
        .with_vault(identities.vault())
        .with_purpose_keys_repository(identities.purpose_keys_repository())
        .build();
    let secure_channels = SecureChannels::builder()
        .await?
        .with_identities(identities.clone())
        .build();
    let identities_creation = identities.identities_creation();

    // Create the CredentialIssuer:
    let options = SecureChannelListenerOptions::new();
    let sc_flow_control_id = options.spawner_flow_control_id();
    secure_channels
        .create_secure_channel_listener(ctx, &auth_identifier, api_worker_addr.clone(), options)
        .await?;
    ctx.flow_controls()
        .add_consumer(auth_worker_addr.clone(), &sc_flow_control_id);
    let auth = CredentialsIssuer::new(members, identities.credentials(), &auth_identifier);
    ctx.start_worker(auth_worker_addr.clone(), auth).await?;

    // Connect to the API channel from the member:
    let e2a = secure_channels
        .create_secure_channel(
            ctx,
            &member_identifier,
            api_worker_addr,
            SecureChannelOptions::new(),
        )
        .await?;
    // Add the member via the enroller's connection:
    // Get a fresh member credential and verify its validity:
    let client = Client::new(&route![e2a, auth_worker_addr], None);
    let credential: CredentialAndPurposeKey =
        client.ask(ctx, Request::post("/")).await?.success()?;

    let exported = member_identity.export()?;

    let imported = identities_creation
        .import(Some(&member_identifier), &exported)
        .await
        .unwrap();
    let data = identities
        .credentials()
        .credentials_verification()
        .verify_credential(Some(&imported), &[auth_identifier.clone()], &credential)
        .await?;
    assert_eq!(
        Some(&b"value".to_vec().into()),
        data.credential_data
            .subject_attributes
            .map
            .get::<ByteSlice>(b"attr".as_slice().into())
    );
    ctx.stop().await
}
