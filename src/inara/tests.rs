use super::*;
use std::time::Duration;

#[test]
fn parses_items_with_split_and_dedup() {
    let html = r#"
    <div id="tab_items">
        <table>
            <tr><th>Item</th><th>c2</th><th>c3</th><th>Location</th></tr>
            <tr><td>Alpha  Item</td><td>x</td><td>y</td><td>A,  B , C</td></tr>
            <tr><td>Alpha  Item</td><td>x</td><td>y</td><td>SHOULD NOT BE USED</td></tr>
            <tr><td>Beta</td><td>x</td><td>y</td><td> </td></tr>
        </table>
    </div>
    <div id="tab_components">
        <table>
            <tr><th>Item</th><th>c2</th><th>c3</th><th>Location</th></tr>
            <tr><td>Gamma</td><td>x</td><td>y</td><td>Z</td></tr>
        </table>
    </div>
    "#;

    let map = parse_material_locations_from_html(html);

    let mut alpha = map.get("Alpha Item").cloned().unwrap_or_default();
    alpha.sort();
    assert_eq!(alpha, vec!["A".to_string(), "B".to_string(), "C".to_string()]);

    let beta = map.get("Beta").cloned().unwrap_or_default();
    assert_eq!(beta, vec![String::new()]);

    let gamma = map.get("Gamma").cloned().unwrap_or_default();
    assert_eq!(gamma, vec!["Z".to_string()]);

    assert_eq!(map.len(), 3);
}

/// Live integration test against https://inara.cz/elite/components/.
#[tokio::test]
async fn reads_real_inara_site() {
    let scraper = Scraper::new();
    let res = tokio::time::timeout(Duration::from_secs(30), scraper.material_locations()).await;
    let map = match res {
        Ok(Ok(m)) => m,
        Ok(Err(e)) => panic!("scrape error: {e}"),
        Err(_) => panic!("timeout fetching inara"),
    };

    assert!(!map.is_empty(), "Expected non-empty map from live site");

    // Ensure Carbon has the expected three locations from the live site.
    let mut carbon = map.get("Carbon").cloned().expect("Carbon present in live data");
    carbon.sort();
    let mut expected = vec![
        "Surface prospecting".to_string(),
        "Mining".to_string(),
        "Mining (Ice rings)".to_string(),
    ];
    expected.sort();
    assert_eq!(carbon, expected, "Carbon locations mismatch");
}


#[tokio::test]
async fn reads_real_inara_items_tab() {
    let scraper = Scraper::new();
    let res = tokio::time::timeout(Duration::from_secs(30), scraper.item_locations()).await;
    let map = match res {
        Ok(Ok(m)) => m,
        Ok(Err(e)) => panic!("scrape error: {e}"),
        Err(_) => panic!("timeout fetching inara"),
    };

    assert!(!map.is_empty(), "Expected non-empty map from items tab");

    // Verify that 'Biological Sample' has the expected locations (case-insensitive)
    let bio = map.get("Biological Sample").cloned().expect("Biological Sample present in items tab");
    let bio_lower: Vec<String> = bio.iter().map(|s| s.to_ascii_lowercase()).collect();
    assert!(bio_lower.iter().any(|s| s == "sto buildings"), "Biological Sample should include 'STO Buildings' (case-insensitive), got: {:?}", bio);
    assert!(bio_lower.iter().any(|s| s == "agri buildings"), "Biological Sample should include 'AGRI Buildings' (case-insensitive), got: {:?}", bio);
}
