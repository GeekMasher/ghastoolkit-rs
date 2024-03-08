use crate::{
    supplychain::{License, Licenses},
    Dependency,
};

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

    /// Extend the list of dependencies with a new list of dependencies
    pub fn extend(&mut self, dependencies: Vec<Dependency>) {
        self.dependencies.extend(dependencies);
    }

    /// Get the length of the list of dependencies
    pub fn len(&self) -> usize {
        self.dependencies.len()
    }

    /// Check if the list of dependencies is empty
    pub fn is_empty(&self) -> bool {
        self.dependencies.is_empty()
    }

    /// Check if the list contains a particular Dependency
    pub fn contains(&self, dependency: &Dependency) -> bool {
        self.dependencies.contains(dependency)
    }

    /// Find a dependency by name
    pub fn find_by_name(&self, name: &str) -> Option<Dependency> {
        self.dependencies.iter().find(|d| d.name == name).cloned()
    }

    /// Find a list of dependencies by names
    pub fn find_by_names(&self, names: &[&str]) -> Vec<Dependency> {
        self.dependencies
            .iter()
            .filter(|d| names.contains(&d.name.as_str()))
            .cloned()
            .collect()
    }

    /// Find a list of dependencies by license
    pub fn find_by_license(&self, license: &License) -> Vec<Dependency> {
        // TODO(geekmasher): support for wildcard licenses
        self.dependencies
            .iter()
            .filter(|d| d.licenses.contains(license))
            .cloned()
            .collect()
    }

    /// Find a list of dependencies by licenses
    pub fn find_by_licenses(&self, licenses: &Licenses) -> Vec<Dependency> {
        // TODO(geekmasher): the clone here is not great, but it's a quick fix for now
        self.dependencies
            .iter()
            .filter(|d| {
                d.licenses
                    .clone()
                    .into_iter()
                    .any(|l| licenses.contains(&l))
            })
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{supplychain::License, Dependencies, Dependency};

    #[test]
    fn test_find_by_name() {
        let mut deps = Dependencies::new();
        deps.extend(vec![
            Dependency::from("pkg:cargo/ghastoolkit-rs@0.2.0"),
            Dependency::from("pkg:pip/ghastoolkit@0.12.0"),
        ]);

        assert_eq!(deps.len(), 2);

        let dep = deps
            .find_by_name("ghastoolkit-rs")
            .expect("Failed to find dependency by name");

        assert_eq!(dep.name, "ghastoolkit-rs");
        assert_eq!(dep.version, Some("0.2.0".to_string()));
    }

    #[test]
    fn test_find_by_license() {
        let mut deps = Dependencies::new();
        deps.extend(vec![
            Dependency::from(("pkg:cargo/ghastoolkit-rs@0.2.0", "MIT")),
            Dependency::from(("pkg:pip/ghastoolkit@0.12.0", "Apache-2.0")),
        ]);
        assert_eq!(deps.len(), 2);

        let deps_license = deps.find_by_license(&License::MIT);
        assert_eq!(deps_license.len(), 1);
        let dep = deps_license
            .first()
            .expect("Failed to find dependency by license");
        assert_eq!(dep.name, "ghastoolkit-rs");
        assert_eq!(dep.manager, "cargo");
    }
}
