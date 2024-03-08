use crate::Dependency;

/// List of Dependencies
#[derive(Debug, Clone, Default)]
pub struct Dependencies {
    dependencies: Vec<Dependency>,
}

impl Iterator for Dependencies {
    type Item = Dependency;

    fn next(&mut self) -> Option<Self::Item> {
        self.dependencies.pop()
    }
}

impl Dependencies {
    /// Create a new list of dependencies
    ///
    /// # Example
    ///
    /// ```rust
    /// use ghastoolkit::{Dependency, Dependencies};
    ///
    /// let mut dependencies = Dependencies::new();
    ///
    /// dependencies.push(Dependency::from("pkg:cargo/ghastoolkit-rs@0.2.0"));
    ///
    /// for dependency in dependencies {
    ///     println!("{}", dependency);
    ///     // Do something with the dependency
    /// }
    /// ```
    pub fn new() -> Self {
        Self {
            dependencies: Vec::new(),
        }
    }

    /// Push a new dependency to the list
    pub fn push(&mut self, dependency: Dependency) {
        self.dependencies.push(dependency);
    }
}
