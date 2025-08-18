use super::*;

#[tokio::test]
async fn test_get_version() {
    let client = ArdentClient::default();
    let version = client.get_version().await.unwrap();
    println!("Ardent version: {:?}", version);
    assert!(!version.version.is_empty());
}

#[tokio::test]
async fn test_get_stats() {
    let client = ArdentClient::default();
    let stats = client.get_stats().await.unwrap();
    println!("Ardent stats: {:?}", stats);
    assert!(stats.systems > 0);
    assert!(stats.stations.stations > 0);
    assert!(stats.trade.markets > 0);
    assert!(stats.trade.orders > 0);
    assert!(!stats.timestamp.is_empty());
}

#[tokio::test]
async fn test_get_station_economies_stats() {
    let client = ArdentClient::default();
    let economies = client.get_station_economies_stats().await.unwrap();
    println!("Station economies: {:?}", economies);
    assert!(!economies.primary.is_empty());
    assert!(!economies.secondary.is_empty());
    assert!(!economies.timestamp.is_empty());
    // Should have common economies like Industrial, Service, etc.
    assert!(economies.primary.contains_key("Industrial") || economies.primary.contains_key("Service"));
    assert!(economies.fleet_carriers > 0);
}

#[tokio::test]
async fn test_get_station_types_stats() {
    let client = ArdentClient::default();
    let types = client.get_station_types_stats().await.unwrap();
    println!("Total station types: {}", types.total);
}

#[tokio::test]
async fn test_get_commodities_report() {
    let client = ArdentClient::default();
    let commodities = client.get_commodities_report().await.unwrap();
    println!("Commodities count: {}", commodities.len());
    assert!(!commodities.is_empty());

    // Check that we have some basic commodity data
    let first_commodity = &commodities[0];
    assert!(!first_commodity.commodity_name.is_empty());
    assert!(!first_commodity.timestamp.is_empty());

    // Check that at least some price data is available
    if let Some(max_buy) = first_commodity.max_buy_price {
        assert!(max_buy > 0);
    }
    if let Some(min_buy) = first_commodity.min_buy_price {
        assert!(min_buy > 0);
    }
    if let Some(avg_buy) = first_commodity.avg_buy_price {
        assert!(avg_buy > 0);
    }

    // Find a non-rare commodity to test stock/demand fields
    if let Some(normal_commodity) = commodities.iter().find(|c| c.rare.is_none() || !c.rare.unwrap()) {
        assert!(normal_commodity.total_stock.is_some());
        assert!(normal_commodity.total_demand.is_some());
    }
}

#[tokio::test]
async fn test_get_commodity_info() {
    let client = ArdentClient::default();
    let result = client.get_commodity_info("Gold").await;
    match result {
        Ok(info) => {
            println!("Gold commodity info: {:?}", info);
            assert_eq!(info.commodity_name, "gold");
        }
        Err(e) => println!("Expected error for commodity info (may not be implemented): {}", e),
    }
}

#[tokio::test]
async fn test_get_commodity_imports() {
    let client = ArdentClient::default();
    let imports = client.get_commodity_imports("Gold", None).await;
    match imports {
        Ok(orders) => {
            println!("Gold imports count: {}", orders.len());
            if !orders.is_empty() {
                let first_order = &orders[0];
                assert!(!first_order.system.is_empty());
                assert!(!first_order.station.is_empty());
                assert!(first_order.price > 0);
            }
        }
        Err(e) => println!("Error getting commodity imports: {}", e),
    }
}

#[tokio::test]
async fn test_get_commodity_exports() {
    let client = ArdentClient::default();
    let exports = client.get_commodity_exports("Gold", None).await;
    match exports {
        Ok(orders) => {
            println!("Gold exports count: {}", orders.len());
            if !orders.is_empty() {
                let first_order = &orders[0];
                assert!(!first_order.system.is_empty());
                assert!(!first_order.station.is_empty());
                assert!(first_order.price > 0);
            }
        }
        Err(e) => println!("Error getting commodity exports: {}", e),
    }
}

#[tokio::test]
async fn test_get_system_info() {
    let client = ArdentClient::default();
    let system_info = client.get_system_info("Sol").await;
    match system_info {
        Ok(info) => {
            println!("Sol system info: {:?}", info);
            assert_eq!(info.name, "Sol");
        }
        Err(e) => println!("Error getting system info: {}", e),
    }
}

#[tokio::test]
async fn test_get_nearby_systems() {
    let client = ArdentClient::default();
    let nearby = client.get_nearby_systems("Sol", Some(20)).await;
    match nearby {
        Ok(systems) => {
            println!("Nearby systems count: {}", systems.len());
            if !systems.is_empty() {
                let first_system = &systems[0];
                assert!(!first_system.name.is_empty());
                assert!(first_system.distance >= 0.0);
            }
        }
        Err(e) => println!("Error getting nearby systems: {}", e),
    }
}

#[tokio::test]
async fn test_get_nearest_service() {
    let client = ArdentClient::default();
    let nearest = client.get_nearest_service("Sol", "Shipyard", None).await;
    match nearest {
        Ok(services) => {
            println!("Nearest shipyard services count: {}", services.len());
            if !services.is_empty() {
                let first_service = &services[0];
                assert!(!first_service.system.is_empty());
                assert!(!first_service.station.is_empty());
                assert_eq!(first_service.service, "Shipyard");
            }
        }
        Err(e) => println!("Error getting nearest service: {}", e),
    }
}

#[tokio::test]
async fn test_get_system_commodities() {
    let client = ArdentClient::default();
    let commodities = client.get_system_commodities("Sol").await;
    match commodities {
        Ok(commodities) => {
            println!("Sol commodities count: {}", commodities.len());
            if !commodities.is_empty() {
                let first_commodity = &commodities[0];
                assert!(!first_commodity.commodity.is_empty());
                assert_eq!(first_commodity.system, "Sol");
            }
        }
        Err(e) => println!("Error getting system commodities: {}", e),
    }
}

#[tokio::test]
async fn test_query_params_construction() {
    let params = CommodityQueryParams {
        min_volume: Some(100),
        min_price: Some(1000),
        max_price: Some(5000),
        fleet_carriers: Some(true),
        max_days_ago: Some(7),
    };

    let query_params = params.to_query_params();
    assert_eq!(query_params.len(), 5);

    let min_volume = query_params.iter().find(|(k, _)| *k == "minVolume").unwrap();
    assert_eq!(min_volume.1, "100");

    let min_price = query_params.iter().find(|(k, _)| *k == "minPrice").unwrap();
    assert_eq!(min_price.1, "1000");

    let max_price = query_params.iter().find(|(k, _)| *k == "maxPrice").unwrap();
    assert_eq!(max_price.1, "5000");

    let fleet_carriers = query_params.iter().find(|(k, _)| *k == "fleetCarriers").unwrap();
    assert_eq!(fleet_carriers.1, "1");

    let max_days = query_params.iter().find(|(k, _)| *k == "maxDaysAgo").unwrap();
    assert_eq!(max_days.1, "7");
}

#[tokio::test]
async fn test_nearby_commodity_query_params_construction() {
    let params = NearbyCommodityQueryParams {
        min_volume: Some(50),
        min_price: Some(500),
        max_price: Some(2000),
        fleet_carriers: Some(false),
        max_distance: Some(25),
        max_days_ago: Some(3),
    };

    let query_params = params.to_query_params();
    assert_eq!(query_params.len(), 6);

    let min_volume = query_params.iter().find(|(k, _)| *k == "minVolume").unwrap();
    assert_eq!(min_volume.1, "50");

    let max_distance = query_params.iter().find(|(k, _)| *k == "maxDistance").unwrap();
    assert_eq!(max_distance.1, "25");

    let fleet_carriers = query_params.iter().find(|(k, _)| *k == "fleetCarriers").unwrap();
    assert_eq!(fleet_carriers.1, "0");
}

#[tokio::test]
async fn test_client_creation() {
    let client = ArdentClient::default();
    assert!(client.base.as_str().starts_with("https://api.ardent-insight.com"));

    let custom_client = ArdentClient::new("https://example.com/api/").unwrap();
    assert_eq!(custom_client.base.as_str(), "https://example.com/api/");
}

#[tokio::test]
async fn test_invalid_base_url() {
    let result = ArdentClient::new("not_a_valid_url");
    assert!(result.is_err());
    match result {
        Err(ArdentError::Url(_)) => (),
        _ => panic!("Expected URL parse error"),
    }
}
