use tari_template_lib::prelude::*;

#[template]
mod template {
    use super::*;

    /// Defines the component state
    pub struct Monerokon {
        counter: u32,
    }

    impl Monerokon {
        /// Construct the component with no args
        pub fn new() -> Component<Self> {
            let state = Self { counter: 0 };

            // Create the component
            Component::new(state).create()
        }

        pub fn counter(&self) -> u32 {
            self.counter
        }

        pub fn increase(&mut self) {
            self.counter += 1
        }
    }
}
