use super::*;

#[tokio::test]
async fn test_get_elite_server_status() {
    let client = EdsmClient::default();
    let status = client.get_elite_server_status().await.unwrap();

    println!("Elite server status: {:?}", status);
    assert!(!status.message.is_empty() || !status.r#type.is_empty());
}

#[tokio::test]
async fn test_get_system() {
    let client = EdsmClient::default();
    let system = client.get_system("Sol").await.unwrap();

    println!("Sol system: {:?}", system);
    assert_eq!(system.name, "Sol");
    // Sol coordinates are well-known: approximately (0, 0, 0)
    assert!((system.coords.x - 0.0).abs() < 1.0);
    assert!((system.coords.y - 0.0).abs() < 1.0);
    assert!((system.coords.z - 0.0).abs() < 1.0);
}

#[tokio::test]
async fn test_get_bodies() {
    let client = EdsmClient::default();
    let bodies = client.get_bodies("Sol").await.unwrap();
    let bodies = bodies.bodies.unwrap();
    println!("Sol bodies count: {}", bodies.len());
    assert!(!bodies.is_empty());
    // Sol should have many bodies including Earth
    let earth = bodies.iter().find(|b| b.name.as_ref() == "Earth").unwrap();
    assert_eq!(earth.name.as_ref(), "Earth");
    assert_eq!(earth.body_type.as_ref(), "Planet");

}

#[tokio::test]
async fn test_get_stations() {
    let client = EdsmClient::default();
    let result = client.get_stations("Sol").await.unwrap();
    let stations = result.stations.unwrap();
    println!("Sol stations count: {}", stations.len());
    assert_eq!(result.name.unwrap().as_ref(), "Sol");
    if !stations.is_empty() {
        let first_station = &stations[0];
        assert!(!first_station.name.is_empty());
    }
}

#[tokio::test]
#[ignore] // this endpoint just doesn't work MOST of the time. It doesn't even return an empty array but instead an empty object.
async fn test_get_sphere_systems() {
    let client = EdsmClient::default();
    let systems = client.get_sphere_systems("Sol", 10.0).await.unwrap();
    println!("Systems within 10 LY of Sol: {}", systems.len());
    assert!(!systems.is_empty());
    // Should include Sol itself or nearby systems
    if let Some(first_system) = systems.first() {
        assert!(!first_system.name.is_empty());
    }
    // Check that all systems are within the specified radius
    for system in &systems {
        let coords = &system.coords;
        let distance = (coords.x.powi(2) + coords.y.powi(2) + coords.z.powi(2)).sqrt();
        assert!(distance <= 10.5); // Allow some tolerance
    }
}

#[tokio::test]
async fn test_get_factions() {
    let client = EdsmClient::default();
    let factions = client.get_factions("Sol").await.unwrap();
            println!("Sol factions count: {}", factions.factions.len());
            if !factions.factions.is_empty() {
                let first_faction = &factions.factions[0];
                assert!(!first_faction.name.is_empty());
                assert!(first_faction.influence >= 0.0);
                assert!(first_faction.influence <= 100.0);
            }
}

#[tokio::test]
async fn test_get_traffic() {
    let client = EdsmClient::default();
    let result = client.get_traffic("Sol").await.unwrap();
    let traffic = result.traffic.unwrap();
    println!("Sol traffic - Day: {}, Week: {}, Total: {}",
             traffic.day,
             traffic.week,
             traffic.total);
    // Traffic counts should be reasonable
    assert!(traffic.total >= traffic.week);
    assert!(traffic.week >= traffic.day);
}

#[tokio::test]
async fn test_get_deaths() {
    let client = EdsmClient::default();
    let result = client.get_deaths("Sol").await.unwrap();
    let deaths = result.deaths.unwrap();
    println!("Sol deaths - Day: {}, Week: {}, Total: {}",
             deaths.day,
             deaths.week,
             deaths.total);
    // Death counts should be reasonable
    assert!(deaths.total >= deaths.week);
    assert!(deaths.week >= deaths.day);

}

#[tokio::test]
async fn test_client_creation() {
    let client = EdsmClient::default();
    assert!(client.base.as_str().starts_with("https://www.edsm.net"));

    let custom_client = EdsmClient::new("https://example.com/api/").unwrap();
    assert_eq!(custom_client.base.as_str(), "https://example.com/api/");
}

#[tokio::test]
async fn test_invalid_base_url() {
    let result = EdsmClient::new("not_a_valid_url");
    assert!(result.is_err());
    match result {
        Err(EdsmError::Url(_)) => (),
        _ => panic!("Expected URL parse error"),
    }
}

#[tokio::test]
async fn test_nonexistent_system() {
    let client = EdsmClient::default();
    let system = client.get_system("ThisSystemShouldNotExist12345").await;
    match system {
        Ok(sys) => {
            panic!("Unexpected success for nonexistent system: {:?}", sys);
            // Some APIs might return empty data instead of error
        }
        Err(e) => {
            println!("Expected error for nonexistent system: {}", e);
        }
    }
}

#[tokio::test]
async fn test_system_with_special_characters() {
    let client = EdsmClient::default();
    // Test with a system name that has special characters
    let system = client.get_system("HIP 22460").await.unwrap();
    println!("System with special chars: {:?}", system);
    assert!(system.name.contains("HIP"));
}