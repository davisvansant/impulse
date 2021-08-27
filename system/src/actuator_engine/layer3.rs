use rand::seq::SliceRandom;
use rand::thread_rng;

use std::net::Ipv4Addr;
use std::u8::{MAX, MIN};
use std::vec::Vec;

use crate::system_error::SystemError;

#[derive(Debug, PartialEq)]
enum Class {
    A,
    B,
    C,
}

impl Class {
    async fn subnet_mask(&self) -> Ipv4Addr {
        match self {
            Class::A => Ipv4Addr::new(255, 0, 0, 0),
            Class::B => Ipv4Addr::new(255, 255, 0, 0),
            Class::C => Ipv4Addr::new(255, 255, 255, 0),
        }
    }

    async fn generate(&self) -> Vec<Ipv4Addr> {
        let mut range = Vec::with_capacity(256);

        for address in MIN..=MAX {
            let new = match self {
                Class::A => Ipv4Addr::new(10, 10, 10, address),
                Class::B => Ipv4Addr::new(172, 31, 10, address),
                Class::C => Ipv4Addr::new(192, 168, 10, address),
            };
            range.push(new);
        }
        range
    }
}

pub struct Layer3 {
    dhcp_enabled: bool,
    class: Class,
    subnet_mask: Ipv4Addr,
    pool: Vec<Ipv4Addr>,
    assigned: Vec<Ipv4Addr>,
}

impl Layer3 {
    pub async fn init() -> Result<Layer3, Box<dyn std::error::Error>> {
        let dhcp_enabled = false;
        let class = Class::B;
        let subnet_mask = class.subnet_mask().await;
        let pool = class.generate().await;
        let assigned = Vec::with_capacity(256);

        Ok(Layer3 {
            dhcp_enabled,
            class,
            subnet_mask,
            pool,
            assigned,
        })
    }

    pub async fn allocate_address(&mut self) -> Result<Ipv4Addr, Box<dyn std::error::Error>> {
        let mut attempts = 0;
        while attempts <= 15 {
            let random_address = self.choose_random_address().await;
            match self.assigned.contains(&random_address) {
                true => {
                    attempts += 1;
                    continue;
                }
                false => {
                    self.assigned.push(random_address);
                    return Ok(random_address);
                }
            }
        }

        let error = SystemError::new("The address pool is exhausted");

        Err(Box::new(error))
    }

    pub async fn reclaim_address(
        &mut self,
        address: &Ipv4Addr,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.assigned.contains(address) {
            if let Ok(index) = self.assigned.binary_search(address) {
                self.assigned.remove(index);
            }
        }

        Ok(())
    }

    async fn choose_random_address(&self) -> Ipv4Addr {
        let mut rng = thread_rng();
        *self.pool.choose(&mut rng).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn class_a_subnet_mask() -> Result<(), Box<dyn std::error::Error>> {
        let test_class_a_subnet_mask = Class::A.subnet_mask().await;
        assert_eq!(test_class_a_subnet_mask.to_string().as_str(), "255.0.0.0");
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn class_b_subnet_mask() -> Result<(), Box<dyn std::error::Error>> {
        let test_class_b_subnet_mask = Class::B.subnet_mask().await;
        assert_eq!(test_class_b_subnet_mask.to_string().as_str(), "255.255.0.0");
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn class_c_subnet_mask() -> Result<(), Box<dyn std::error::Error>> {
        let test_class_c_subnet_mask = Class::C.subnet_mask().await;
        assert_eq!(
            test_class_c_subnet_mask.to_string().as_str(),
            "255.255.255.0",
        );
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn class_a_generate() -> Result<(), Box<dyn std::error::Error>> {
        let test_class_a_generate = Class::A.generate().await;
        assert_eq!(test_class_a_generate.len(), 256);
        assert_eq!(test_class_a_generate.capacity(), 256);
        for address in test_class_a_generate.iter() {
            let test_address = Ipv4Addr::new(10, 10, 10, address.octets()[3]);
            assert!(address.is_private());
            assert!(!address.is_broadcast());
            assert!(!address.is_documentation());
            assert!(!address.is_link_local());
            assert!(!address.is_multicast());
            assert_eq!(address, &test_address);
        }
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn class_b_generate() -> Result<(), Box<dyn std::error::Error>> {
        let test_class_b_generate = Class::B.generate().await;
        assert_eq!(test_class_b_generate.len(), 256);
        assert_eq!(test_class_b_generate.capacity(), 256);
        for address in test_class_b_generate.iter() {
            let test_address = Ipv4Addr::new(172, 31, 10, address.octets()[3]);
            assert!(address.is_private());
            assert!(!address.is_broadcast());
            assert!(!address.is_documentation());
            assert!(!address.is_link_local());
            assert!(!address.is_multicast());
            assert_eq!(address, &test_address);
        }
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn class_c_generate() -> Result<(), Box<dyn std::error::Error>> {
        let test_class_c_generate = Class::C.generate().await;
        assert_eq!(test_class_c_generate.len(), 256);
        assert_eq!(test_class_c_generate.capacity(), 256);
        for address in test_class_c_generate.iter() {
            let test_address = Ipv4Addr::new(192, 168, 10, address.octets()[3]);
            assert!(address.is_private());
            assert!(!address.is_broadcast());
            assert!(!address.is_documentation());
            assert!(!address.is_link_local());
            assert!(!address.is_multicast());
            assert_eq!(address, &test_address);
        }
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn init() -> Result<(), Box<dyn std::error::Error>> {
        let test_layer3 = Layer3::init().await?;
        assert!(!test_layer3.dhcp_enabled);
        assert_eq!(test_layer3.class, Class::B);
        assert_eq!(test_layer3.subnet_mask.to_string().as_str(), "255.255.0.0");
        assert_eq!(test_layer3.pool.len(), 256);
        assert_eq!(test_layer3.pool.capacity(), 256);
        assert_eq!(test_layer3.assigned.len(), 0);
        assert_eq!(test_layer3.assigned.capacity(), 256);
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn allocate_address() -> Result<(), Box<dyn std::error::Error>> {
        let mut test_layer3 = Layer3::init().await?;
        assert_eq!(test_layer3.pool.len(), 256);
        assert_eq!(test_layer3.assigned.len(), 0);
        assert!(test_layer3.allocate_address().await.is_ok());
        assert_eq!(test_layer3.assigned.len(), 1);
        assert!(test_layer3.allocate_address().await.is_ok());
        assert_eq!(test_layer3.assigned.len(), 2);
        assert!(test_layer3.allocate_address().await.is_ok());
        assert_eq!(test_layer3.assigned.len(), 3);
        assert!(test_layer3.allocate_address().await.is_ok());
        assert_eq!(test_layer3.assigned.len(), 4);
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn allocate_address_error() -> Result<(), Box<dyn std::error::Error>> {
        let mut test_layer3 = Layer3::init().await?;
        assert_eq!(test_layer3.pool.len(), 256);
        assert_eq!(test_layer3.assigned.len(), 0);
        for address in &test_layer3.pool {
            test_layer3.assigned.push(*address);
        }
        assert_eq!(test_layer3.assigned.len(), 256);
        assert!(test_layer3.allocate_address().await.is_err());
        assert_eq!(test_layer3.assigned.len(), 256);
        assert!(test_layer3.allocate_address().await.is_err());
        assert_eq!(test_layer3.assigned.len(), 256);
        assert!(test_layer3.allocate_address().await.is_err());
        assert_eq!(test_layer3.assigned.len(), 256);
        assert!(test_layer3.allocate_address().await.is_err());
        assert_eq!(test_layer3.assigned.len(), 256);
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn reclaim_address() -> Result<(), Box<dyn std::error::Error>> {
        let mut test_layer3 = Layer3::init().await?;
        assert_eq!(test_layer3.pool.len(), 256);
        assert_eq!(test_layer3.assigned.len(), 0);
        for address in &test_layer3.pool {
            test_layer3.assigned.push(*address);
        }
        assert_eq!(test_layer3.assigned.len(), 256);
        let test_cloned_pool = test_layer3.pool.clone();
        for address in &test_cloned_pool {
            test_layer3.reclaim_address(address).await?;
        }
        assert_eq!(test_layer3.assigned.len(), 0);
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn choose_random_address() -> Result<(), Box<dyn std::error::Error>> {
        let test_layer3 = Layer3::init().await?;
        assert_eq!(test_layer3.pool.len(), 256);
        assert_eq!(test_layer3.pool.capacity(), 256);
        let test_random_address = test_layer3.choose_random_address().await;
        assert!(test_random_address.is_private());
        Ok(())
    }
}
