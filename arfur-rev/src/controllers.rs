pub mod sparkmax {
    use std::ffi::c_int;

    use arfur_wpilib::robot::Robot;

    use crate::ffi::root::rev::{CANSparkMax, CANSparkMax_Set};

    /// A handle to a REV CAN SparkMax motor controller.
    #[derive(Debug)]
    pub struct SparkMax {
        handle: CANSparkMax,
    }

    impl SparkMax {
        /// Create a new CAN brushless SparkMax.
        ///
        /// # Safety
        /// `id` is a valid CAN id, the motor type is internally ensured.
        pub fn new(_: Robot, id: i32) -> Self {
            let handle = unsafe { CANSparkMax::new(id as c_int, 0 as c_int) };

            Self { handle }
        }

        #[tracing::instrument]
        pub fn reset(&mut self) {
            unsafe {
                self.handle._base.RestoreFactoryDefaults(true);
            }
        }

        /// Set the percentage output.
        ///
        /// Safety: the percentage is a number from -1 to 1.
        #[tracing::instrument]
        pub fn set_percentage(&mut self, percentage: f64) {
            unsafe {
                CANSparkMax_Set(
                    &mut self.handle as *mut _ as *mut std::ffi::c_void,
                    percentage,
                );
            }
        }

        #[tracing::instrument]
        pub fn get_encoder(&self, encoder_type: Optional<i32>, counts_per_rev: Optional<i32>) -> f64 {
            unsafe { self.handle.GetEncoder(encoder_type, counts_per_rev.unwrap_or(42)) }
        }

    }
}
