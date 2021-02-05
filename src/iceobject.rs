use async_trait::async_trait;

/// The `IceObject` trait is a base trait for all
/// ice interfaces. It implements functions that
/// are equal to all ice interfaces.
#[async_trait]
pub trait IceObject {
    async fn ice_ping(&mut self) -> Result<(), Box<dyn std::error::Error + Sync + Send>>;
    async fn ice_is_a(&mut self) -> Result<bool, Box<dyn std::error::Error + Sync + Send>>;
    async fn ice_id(&mut self) -> Result<String, Box<dyn std::error::Error + Sync + Send>>;
    async fn ice_ids(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error + Sync + Send>>;
}