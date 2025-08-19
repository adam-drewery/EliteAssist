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
    let info = client.get_commodity_info("Gold").await.unwrap();

    println!("Gold commodity info: {:?}", info);
    assert_eq!(info.commodity_name, "gold");
}

#[tokio::test]
async fn test_get_commodity_imports() {
    let client = ArdentClient::default();
    let orders = client.get_commodity_imports("Gold", None).await.unwrap();
    println!("Gold imports count: {}", orders.len());
    if !orders.is_empty() {
        let first_order = &orders[0];
        assert!(!first_order.commodity_name.is_empty());
        assert!(first_order.sell_price > 0);
    }
}

#[tokio::test]
async fn test_get_commodity_exports() {
    let client = ArdentClient::default();
    let orders = client.get_commodity_exports("Gold", None).await.unwrap();

    println!("Gold exports count: {}", orders.len());
    if !orders.is_empty() {
        let first_order = &orders[0];
        assert!(!first_order.commodity_name.is_empty());
        assert!(first_order.buy_price > 0);
    }

}

#[tokio::test]
async fn test_get_system_info() {
    let client = ArdentClient::default();
    let info = client.get_system_info("Sol").await.unwrap();

    println!("Sol system info: {:?}", info.system_address);
    assert_eq!(info.system_name, "Sol");
    assert_ne!(info.system_address, 0);

}

#[tokio::test]
async fn test_get_nearby_systems() {
    let client = ArdentClient::default();
    let systems = client.get_nearby_systems("Sol", Some(20)).await.unwrap();
        println!("Nearby systems count: {}", systems.len());
        if !systems.is_empty() {
            let first_system = &systems[0];
            assert!(!first_system.name.is_empty());
            assert!(first_system.distance >= 0.0);
        }
}

#[tokio::test]
async fn test_get_nearest_service() {
    let client = ArdentClient::default();
    let services = client.get_nearest_service("Sol", "shipyard", None).await.unwrap();

    println!("Nearest shipyard services count: {}", services.len());
    if !services.is_empty() {
        let first_service = &services[0];
        assert!(!first_service.system_name.is_empty());
        assert!(!first_service.station_name.is_empty());
    }

}

#[tokio::test]
async fn test_get_system_commodities() {
    let client = ArdentClient::default();
    let commodities = client.get_system_commodities("Sol").await.unwrap();

    println!("Sol commodities count: {}", commodities.len());
    if !commodities.is_empty() {
        let first_commodity = &commodities[0];
        assert!(!first_commodity.station_name.is_empty());
        assert!(!first_commodity.station_type.is_empty());
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

#[tokio::test]
async fn test_get_system_info_by_address() {
    let client = ArdentClient::default();
    // Sol's system address is known: 10477373803
    let info = client.get_system_info_by_address(10477373803).await.unwrap();
    
    println!("Sol system info by address: {:?}", info.system_name);
    assert_eq!(info.system_name, "Sol");
    assert_eq!(info.system_address, 10477373803);
}

#[tokio::test]
async fn test_get_system_commodity_imports() {
    let client = ArdentClient::default();
    let orders = client.get_system_commodity_imports("Sol", None).await.unwrap();
    
    println!("Sol commodity imports count: {}", orders.len());
    if !orders.is_empty() {
        let first_order = &orders[0];
        assert!(!first_order.commodity_name.is_empty());
        assert!(first_order.sell_price > 0);
    }
}

#[tokio::test]
async fn test_get_system_commodity_exports() {
    let client = ArdentClient::default();
    let orders = client.get_system_commodity_exports("Sol", None).await.unwrap();
    
    println!("Sol commodity exports count: {}", orders.len());
    if !orders.is_empty() {
        let first_order = &orders[0];
        assert!(!first_order.commodity_name.is_empty());
        assert!(first_order.buy_price > 0);
    }
}

#[tokio::test]
async fn test_get_system_commodity_data() {
    let client = ArdentClient::default();
    let data = client.get_system_commodity_data("Sol", "Gold", Some(30)).await.unwrap();

    println!("Sol Gold commodity data: {:?}", data);

    let data = &data[0];

    assert_eq!(data.commodity_name, "gold");
}

#[tokio::test]
async fn test_get_nearby_commodity_imports() {
    let client = ArdentClient::default();
    let params = NearbyCommodityQueryParams {
        min_volume: Some(10),
        min_price: None,
        max_price: None,
        fleet_carriers: Some(false),
        max_distance: Some(50),
        max_days_ago: Some(7),
    };
    let orders = client.get_nearby_commodity_imports("Sol", "Gold", Some(params)).await.unwrap();
    
    println!("Nearby Gold imports count: {}", orders.len());
    if !orders.is_empty() {
        let first_order = &orders[0];
        assert!(!first_order.commodity_name.is_empty());
        assert!(first_order.sell_price > 0);
        // Check that the system is within the specified distance
        let distance = (first_order.system_x.powi(2) + first_order.system_y.powi(2) + first_order.system_z.powi(2)).sqrt();
        assert!(distance <= 55.0); // Allow some tolerance
    }
}

#[tokio::test]
async fn test_get_nearby_commodity_exports() {
    let client = ArdentClient::default();
    let params = NearbyCommodityQueryParams {
        min_volume: Some(10),
        min_price: None,
        max_price: None,
        fleet_carriers: Some(false),
        max_distance: Some(50),
        max_days_ago: Some(7),
    };
    let orders = client.get_nearby_commodity_exports("Sol", "Gold", Some(params)).await.unwrap();
    
    println!("Nearby Gold exports count: {}", orders.len());
    if !orders.is_empty() {
        let first_order = &orders[0];
        assert!(!first_order.commodity_name.is_empty());
        assert!(first_order.buy_price > 0);
        // Check that the system is within the specified distance
        let distance = (first_order.system_x.powi(2) + first_order.system_y.powi(2) + first_order.system_z.powi(2)).sqrt();
        assert!(distance <= 55.0); // Allow some tolerance
    }
}

#[tokio::test]
async fn test_get_market_commodity_data() {
    let client = ArdentClient::default();

    // Find a market that exports Gold to get a valid market_id for the test
    let exports = client.get_commodity_exports("Gold", None).await.unwrap();
    if let Some(first) = exports.first() {
        let market_id = first.market_id;
        let data = client
            .get_market_commodity_data(market_id, "Gold")
            .await
            .unwrap();

        println!(
            "Market commodity data for Gold at market {}: {:?}",
            market_id, data
        );

        // Basic invariants
        assert_eq!(data.market_id, market_id);
        assert_eq!(data.commodity_name, "gold");
    } else {
        // If no Gold export data is available, avoid failing the test to reduce flakiness
        println!("No Gold export orders available to test get_market_commodity_data");
    }
}
