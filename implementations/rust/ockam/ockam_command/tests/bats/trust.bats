#!/bin/bash

# ===== SETUP

setup() {
  load load/base.bash
  load load/orchestrator.bash
  load_bats_ext
  setup_home_dir
}

teardown() {
  teardown_home_dir
}

# ===== TESTS

@test "trust - no trust; everything is accepted" {
  run_success "$OCKAM" identity create m1
  run_success "$OCKAM" node create n1 --identity m1

  run_success "$OCKAM" identity create m2
  run_success "$OCKAM" node create n2 --identity m2

  run_success bash -c "$OCKAM secure-channel create --from /node/n1 --to /node/n2/service/api \
        | $OCKAM message send hello --from /node/n1 --to -/service/echo"
}

#FIXME
#@test "trust - trust with an online authority; Credential Exchange is performed" {
#  auth_port="$(random_port)"
#  node_port="$(random_port)"
#  $OCKAM identity create alice
#  $OCKAM identity create bob
#  $OCKAM identity create attacker
#  $OCKAM identity create authority
#  bob_id=$($OCKAM identity show bob)
#  alice_id=$($OCKAM identity show alice)
#  authority_identity=$($OCKAM identity show --full --encoding hex authority)
#
#  trusted="{\"$bob_id\": {}, \"$alice_id\": {}}"
#  run_success "$OCKAM" authority create --identity authority --tcp-listener-address="127.0.0.1:$auth_port" --trusted-identities "$trusted"
#  assert_success
#  sleep 1
#
#  authority_route="/dnsaddr/127.0.0.1/tcp/$auth_port/service/api"
#  run_success "$OCKAM" trust-context create test-context --id test-context --authority-identity $authority_identity --authority-route $authority_route
#  run_success "$OCKAM" node create --identity alice --tcp-listener-address 127.0.0.1:$node_port --trust-context test-context
#  sleep 1
#
#  # send a message to alice using the trust context
#  msg=$(random_str)
#  run_success "$OCKAM" message send --timeout 2 --identity bob --to /dnsaddr/127.0.0.1/tcp/$node_port/secure/api/service/echo --trust-context test-context $msg
#  assert_output "$msg"
#
#  # send a message to authority node echo service to make sure we can use it as a healthcheck endpoint
#  run_success "$OCKAM" message send --timeout 2 --identity bob --to "/dnsaddr/127.0.0.1/tcp/$auth_port/secure/api/service/echo" $msg
#  assert_output "$msg"
#
#  run_failure "$OCKAM" message send --timeout 2 --identity attacker --to /dnsaddr/127.0.0.1/tcp/$node_port/secure/api/service/echo --trust-context test-context $msg
#  run_failure "$OCKAM" message send --timeout 2 --identity attacker --to /dnsaddr/127.0.0.1/tcp/$node_port/secure/api/service/echo --trust-context $msg
#}
