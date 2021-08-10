use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;
use rand::thread_rng;

pub struct LayerTwo {
    rng: ThreadRng,
    uniform: Uniform<u32>,
}

impl LayerTwo {
    pub async fn init() -> Result<LayerTwo, Box<dyn std::error::Error>> {
        let rng = thread_rng();
        let uniform = Uniform::new_inclusive(0, 15);

        Ok(LayerTwo { rng, uniform })
    }

    pub async fn generate_mac_address(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let (first_octet_digit_one, _) = self.generate_digit().await;
        let (second_octet_digit_one, second_octet_digit_two) = self.generate_digit().await;
        let (third_octet_digit_one, third_octet_digit_two) = self.generate_digit().await;
        let (fourth_octet_digit_one, fourth_octet_digit_two) = self.generate_digit().await;
        let (fifth_octet_digit_one, fifth_octet_digit_two) = self.generate_digit().await;
        let (sixth_octet_digit_one, sixth_octet_digit_two) = self.generate_digit().await;

        let mac_address = format!(
            "{}2:{}{}:{}{}:{}{}:{}{}:{}{}",
            first_octet_digit_one,
            second_octet_digit_one,
            second_octet_digit_two,
            third_octet_digit_one,
            third_octet_digit_two,
            fourth_octet_digit_one,
            fourth_octet_digit_two,
            fifth_octet_digit_one,
            fifth_octet_digit_two,
            sixth_octet_digit_one,
            sixth_octet_digit_two,
        );

        Ok(mac_address)
    }

    async fn generate_digit(&mut self) -> (String, String) {
        let octet: Vec<u32> = self.uniform.sample_iter(&mut self.rng).take(2).collect();

        let octet_digit_one = char::from_digit(octet[0], 16)
            .unwrap()
            .to_uppercase()
            .to_string();

        let octet_digit_two = char::from_digit(octet[1], 16)
            .unwrap()
            .to_uppercase()
            .to_string();

        (octet_digit_one, octet_digit_two)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn init() -> Result<(), Box<dyn std::error::Error>> {
        let test_layer_two = LayerTwo::init().await;
        assert!(test_layer_two.is_ok());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn generate_mac_address() -> Result<(), Box<dyn std::error::Error>> {
        let mut test_layer_two = LayerTwo::init().await?;
        let test_mac_address = test_layer_two.generate_mac_address().await?;
        assert_eq!(test_mac_address.len(), 17);
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn generate_digit() -> Result<(), Box<dyn std::error::Error>> {
        let mut test_layer_two = LayerTwo::init().await?;
        let (test_digit_one, test_digit_two) = test_layer_two.generate_digit().await;
        assert_eq!(test_digit_one.len(), 1);
        assert_eq!(test_digit_two.len(), 1);
        Ok(())
    }
}
