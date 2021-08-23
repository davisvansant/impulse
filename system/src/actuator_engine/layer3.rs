pub struct Layer3 {}

impl Layer3 {
    pub async fn init() -> Result<Layer3, Box<dyn std::error::Error>> {
        Ok(Layer3 {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn init() -> Result<(), Box<dyn std::error::Error>> {
        let test_layer3 = Layer3::init().await;
        assert!(test_layer3.is_ok());
        Ok(())
    }
}
