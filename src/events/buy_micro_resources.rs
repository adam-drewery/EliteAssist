use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BuyMicroResources {
    
    pub timestamp: String,
    
    #[serde(rename = "Name")]
    pub name: String,
    
    #[serde(rename = "Name_Localised")]
    pub name_localised: String,
    
    #[serde(rename = "Category")]
    pub category: String,
    
    #[serde(rename = "Count")]
    pub count: u32,
    
    #[serde(rename = "Price")]
    pub price: u32,
    
    #[serde(rename = "MarketID")]
    pub market_id: u64,
}
