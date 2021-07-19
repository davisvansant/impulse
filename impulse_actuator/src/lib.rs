pub struct Actuator {}

impl Actuator {
    pub async fn init() -> Result<Actuator, Box<dyn std::error::Error>> {
        Ok(Actuator {})
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
