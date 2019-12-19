use super::{TEST_RETRIES, TEST_TIMEOUT};

use crate::data::{Build, FedoraRelease};
use crate::query::{BuildQuery, BuildNVRQuery};
use crate::service::BodhiServiceBuilder;

#[test]
#[ignore]
fn deserialize_f32() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only builds for one release, and deserialize them
    bodhi.query(&BuildQuery::new().releases(FedoraRelease::F32)).unwrap();
}

#[test]
fn deserialize_f32c() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only builds for one release, and deserialize them
    bodhi.query(&BuildQuery::new().releases(FedoraRelease::F32C)).unwrap();
}

#[test]
#[ignore]
fn deserialize_f31() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only builds for one release, and deserialize them
    bodhi.query(&BuildQuery::new().releases(FedoraRelease::F31)).unwrap();
}

#[test]
fn deserialize_f31c() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only builds for one release, and deserialize them
    bodhi.query(&BuildQuery::new().releases(FedoraRelease::F31C)).unwrap();
}

#[test]
fn deserialize_f31f() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only builds for one release, and deserialize them
    bodhi.query(&BuildQuery::new().releases(FedoraRelease::F31F)).unwrap();
}

#[test]
fn deserialize_f31m() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only builds for one release, and deserialize them
    bodhi.query(&BuildQuery::new().releases(FedoraRelease::F31M)).unwrap();
}

#[test]
#[ignore]
fn deserialize_f30() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only builds for one release, and deserialize them
    bodhi.query(&BuildQuery::new().releases(FedoraRelease::F30)).unwrap();
}

#[test]
fn deserialize_f30c() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only builds for one release, and deserialize them
    bodhi.query(&BuildQuery::new().releases(FedoraRelease::F30C)).unwrap();
}

#[test]
fn deserialize_f30f() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only builds for one release, and deserialize them
    bodhi.query(&BuildQuery::new().releases(FedoraRelease::F30F)).unwrap();
}

#[test]
fn deserialize_f30m() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only builds for one release, and deserialize them
    bodhi.query(&BuildQuery::new().releases(FedoraRelease::F30M)).unwrap();
}

#[test]
#[ignore]
fn deserialize_f29() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only builds for one release, and deserialize them
    bodhi.query(&BuildQuery::new().releases(FedoraRelease::F29)).unwrap();
}

#[test]
fn deserialize_f29c() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only builds for one release, and deserialize them
    bodhi.query(&BuildQuery::new().releases(FedoraRelease::F29C)).unwrap();
}

#[test]
fn deserialize_f29f() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only builds for one release, and deserialize them
    bodhi.query(&BuildQuery::new().releases(FedoraRelease::F29F)).unwrap();
}

#[test]
fn deserialize_f29m() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only builds for one release, and deserialize them
    bodhi.query(&BuildQuery::new().releases(FedoraRelease::F29M)).unwrap();
}

#[test]
#[ignore]
fn deserialize_f28() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only builds for one release, and deserialize them
    bodhi.query(&BuildQuery::new().releases(FedoraRelease::F28)).unwrap();
}

#[test]
fn deserialize_f28c() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only builds for one release, and deserialize them
    bodhi.query(&BuildQuery::new().releases(FedoraRelease::F28C)).unwrap();
}

#[test]
fn deserialize_f28m() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only builds for one release, and deserialize them
    bodhi.query(&BuildQuery::new().releases(FedoraRelease::F28M)).unwrap();
}

#[test]
#[ignore]
fn deserialize_f27() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only builds for one release, and deserialize them
    bodhi.query(&BuildQuery::new().releases(FedoraRelease::F27)).unwrap();
}

#[test]
fn deserialize_f27m() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only builds for one release, and deserialize them
    bodhi.query(&BuildQuery::new().releases(FedoraRelease::F27M)).unwrap();
}

#[test]
#[ignore]
fn deserialize_f26() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only builds for one release, and deserialize them
    bodhi.query(&BuildQuery::new().releases(FedoraRelease::F26)).unwrap();
}

#[test]
#[ignore]
fn deserialize_f25() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only builds for one release, and deserialize them
    bodhi.query(&BuildQuery::new().releases(FedoraRelease::F25)).unwrap();
}

#[test]
#[ignore]
fn deserialize_f24() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only builds for one release, and deserialize them
    bodhi.query(&BuildQuery::new().releases(FedoraRelease::F24)).unwrap();
}

#[test]
#[ignore]
fn deserialize_f23() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only builds for one release, and deserialize them
    bodhi.query(&BuildQuery::new().releases(FedoraRelease::F23)).unwrap();
}

#[test]
#[ignore]
fn deserialize_f22() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only builds for one release, and deserialize them
    bodhi.query(&BuildQuery::new().releases(FedoraRelease::F22)).unwrap();
}

#[test]
#[ignore]
fn deserialize_f21() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only builds for one release, and deserialize them
    bodhi.query(&BuildQuery::new().releases(FedoraRelease::F21)).unwrap();
}

#[test]
#[ignore]
fn deserialize_epel8() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only builds for one release, and deserialize them
    bodhi.query(&BuildQuery::new().releases(FedoraRelease::EPEL8)).unwrap();
}

#[test]
fn deserialize_epel8m() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only builds for one release, and deserialize them
    bodhi.query(&BuildQuery::new().releases(FedoraRelease::EPEL8M)).unwrap();
}

#[test]
#[ignore]
fn deserialize_epel7() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only builds for one release, and deserialize them
    bodhi.query(&BuildQuery::new().releases(FedoraRelease::EPEL7)).unwrap();
}

#[test]
fn nvr_query_some() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    let build: Option<Build> = bodhi.query(
        &BuildNVRQuery::new(String::from("rust-1.34.2-1.fc30"))
    ).unwrap();

    assert!(build.is_some());
}

#[test]
fn nvr_query_none() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    let build: Option<Build> = bodhi.query(
        &BuildNVRQuery::new(String::from("this-doesnt-exist-1-1.fc30"))
    ).unwrap();

    assert!(build.is_none());
}
