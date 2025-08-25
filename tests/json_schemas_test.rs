// Integration tests for codegen/events/json.rs deserializing selected JSON schema files

#[path = "../codegen/events/json.rs"]
mod json;

use std::path::PathBuf;

fn schema_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir.join("journal-schemas").join("schemas")
}

#[test]
fn deserialize_approach_settlement_schema() {
    let schemas = json::load_schemas(&schema_root()).expect("schemas to load");
    let schema = schemas
        .into_iter()
        .find(|s| s.title == "ApproachSettlement")
        .expect("ApproachSettlement schema found");

    // Core expectations
    assert_eq!(schema.r#type, "object");
    assert_eq!(
        schema.description,
        "When written: when approaching a planetary settlement"
    );

    // Required fields
    for req in ["Name", "SystemAddress", "BodyID", "BodyName"] {
        assert!(schema.required.iter().any(|r| r == req), "missing required: {req}");
    }

    // Properties we expect to exist
    for prop in [
        "Name",
        "SystemAddress",
        "BodyID",
        "BodyName",
        "Latitude",
        "Longitude",
    ] {
        assert!(schema.properties.contains_key(prop), "missing property: {prop}");
    }
}

#[test]
fn deserialize_technology_broker_schema() {
    let schemas = json::load_schemas(&schema_root()).expect("schemas to load");
    let schema = schemas
        .into_iter()
        .find(|s| s.title == "TechnologyBroker")
        .expect("TechnologyBroker schema found");

    // Core expectations
    assert_eq!(schema.r#type, "object");
    assert_eq!(
        schema.description,
        "When written: when using the Technology Broker to unlock new purchasable technology"
    );

    // Required fields
    for req in [
        "BrokerType",
        "MarketID",
        "ItemsUnlocked",
        "Commodities",
        "Materials",
    ] {
        assert!(schema.required.iter().any(|r| r == req), "missing required: {req}");
    }

    // Properties we expect to exist
    for prop in ["BrokerType", "MarketID", "ItemsUnlocked", "Commodities", "Materials"] {
        assert!(schema.properties.contains_key(prop), "missing property: {prop}");
    }
}

#[test]
fn deserialize_powerplay_voucher_schema() {
    let schemas = json::load_schemas(&schema_root()).expect("schemas to load");
    let schema = schemas
        .into_iter()
        .find(|s| s.title == "PowerplayVoucher")
        .expect("PowerplayVoucher schema found");

    // Core expectations
    assert_eq!(schema.r#type, "object");
    assert_eq!(
        schema.description,
        "When written: when receiving payment for powerplay combat"
    );

    // Required fields
    for req in ["Power", "Systems"] {
        assert!(schema.required.iter().any(|r| r == req), "missing required: {req}");
    }

    // Properties we expect to exist
    for prop in ["Power", "Systems"] {
        assert!(schema.properties.contains_key(prop), "missing property: {prop}");
    }
}
