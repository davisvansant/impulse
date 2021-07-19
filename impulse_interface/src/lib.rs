pub struct Interface {}

impl Interface {
    pub async fn init() -> Result<Interface, Box<dyn std::error::Error>> {
        Ok(Interface {})
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
