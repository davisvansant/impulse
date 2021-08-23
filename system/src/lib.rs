pub mod actuator_client;
pub mod actuator_engine;
pub mod external_interface;
pub mod internal_interface;

pub const IMPULSE_ACTUATOR: &str = ":: i m p u l s e _ a c t u a t o r >";
pub const IMPULSE_INTERFACE: &str = ":: i m p u l s e _ i n t e r f a c e >";

pub(crate) mod impulse {
    pub(crate) mod external {
        pub(crate) mod v010 {
            include!("../../proto/impulse.external.v010.rs");
        }
    }

    pub(crate) mod internal {
        pub(crate) mod v010 {
            include!("../../proto/impulse.internal.v010.rs");
        }
    }

    pub(crate) mod shared {
        pub(crate) mod v010 {
            include!("../../proto/impulse.shared.v010.rs");
        }
    }
}
