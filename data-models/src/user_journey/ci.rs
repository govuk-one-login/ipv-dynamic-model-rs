#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum CiState {
    Unmitigated,
    Mitigated,
}

/// In case we want to change the type used here
pub type CiLabel = String;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Ci {
    label: CiLabel,
    state: CiState,
}

impl Ci {
    /// Create a new [`CI`] with a given label. [`CI`]s are always initialized unmitigated.
    /// ```
    /// use data_models::user_journey::ci::{Ci, CiState};
    ///
    /// let ci = Ci::new("CI-123");
    /// assert_eq!(ci.label(), "CI-123");
    /// assert_eq!(ci.state(), CiState::Unmitigated);
    /// ```
    #[must_use]
    pub fn new<L: Into<CiLabel>>(label: L) -> Self {
        Self {
            label: label.into(),
            state: CiState::Unmitigated,
        }
    }

    /// Change the state of a [`CI`] to [`CiState::Mitigated`].
    /// ```
    /// use data_models::user_journey::ci::{Ci, CiState};
    ///
    /// let mut ci = Ci::new("CI-123");
    /// assert_eq!(ci.state(), CiState::Unmitigated);
    /// ci.mitigate();
    /// assert_eq!(ci.state(), CiState::Mitigated);
    /// ```
    ///
    /// Note, mitigating a mitigated CI has no affect
    /// ```
    /// # use data_models::user_journey::ci::{Ci, CiState};
    /// #
    /// # let mut ci = Ci::new("CI-123");
    /// # assert_eq!(ci.state(), CiState::Unmitigated);
    /// # ci.mitigate();
    /// assert_eq!(ci.state(), CiState::Mitigated);
    /// ci.mitigate();
    /// assert_eq!(ci.state(), CiState::Mitigated);
    /// ```
    pub fn mitigate(&mut self) {
        self.state = CiState::Mitigated;
    }

    /// Gets the label of the CI
    /// ```
    /// use data_models::user_journey::ci::Ci;
    ///
    /// let mut ci = Ci::new("CI-123");
    /// assert_eq!(ci.label(), "CI-123");
    /// ```
    #[must_use]
    pub fn label(&self) -> &CiLabel {
        &self.label
    }

    /// Gets the state of the CI
    /// ```
    /// use data_models::user_journey::ci::{Ci, CiState};
    ///
    /// let mut ci = Ci::new("CI-123");
    /// assert_eq!(ci.state(), CiState::Unmitigated);
    /// ```
    #[must_use]
    pub fn state(&self) -> CiState {
        self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ci = Ci::new("CI-123");
        assert_eq!(ci.label(), "CI-123");
        assert_eq!(ci.state(), CiState::Unmitigated);
    }
    #[test]
    fn test_mitigate() {
        let mut ci = Ci::new("CI-123");
        assert_eq!(ci.state(), CiState::Unmitigated);
        ci.mitigate();
        assert_eq!(ci.state(), CiState::Mitigated);
        // mitigating mitigated doesn't do anything
        ci.mitigate();
        assert_eq!(ci.state(), CiState::Mitigated);
    }

    #[test]
    fn test_label() {
        let mut ci = Ci::new("CI-123");
        assert_eq!(ci.label(), "CI-123");
    }

    #[test]
    fn test_state() {
        let mut ci = Ci::new("CI-123");
        assert_eq!(ci.state(), CiState::Unmitigated);
    }
}
