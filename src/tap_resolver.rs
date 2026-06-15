#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Eisu,
    Kana,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    LeftCommand,
    RightCommand,
}

impl Key {
    fn action(self) -> Action {
        match self {
            Key::LeftCommand => Action::Eisu,
            Key::RightCommand => Action::Kana,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Input {
    Down(Key),
    Up(Key),
    Other,
}

#[derive(Default)]
pub struct TapResolver {
    hold: Option<Key>,
    combo: bool,
}

impl TapResolver {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn resolve(&mut self, input: Input) -> Option<Action> {
        match input {
            Input::Down(key) => {
                if self.hold.is_none() {
                    self.hold = Some(key);
                    self.combo = false;
                } else {
                    self.combo = true;
                }
                None
            }
            Input::Other => {
                if self.hold.is_some() {
                    self.combo = true;
                }
                None
            }
            Input::Up(key) => {
                if self.hold != Some(key) {
                    return None;
                }
                let clean = !self.combo;
                self.hold = None;
                self.combo = false;
                if clean { Some(key.action()) } else { None }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_left_command_tap_emits_eisu() {
        let mut resolver = TapResolver::new();
        assert_eq!(resolver.resolve(Input::Down(Key::LeftCommand)), None);
        assert_eq!(
            resolver.resolve(Input::Up(Key::LeftCommand)),
            Some(Action::Eisu)
        );
    }

    #[test]
    fn single_right_command_tap_emits_kana() {
        let mut resolver = TapResolver::new();
        assert_eq!(resolver.resolve(Input::Down(Key::RightCommand)), None);
        assert_eq!(
            resolver.resolve(Input::Up(Key::RightCommand)),
            Some(Action::Kana)
        );
    }

    #[test]
    fn command_with_other_key_does_not_emit() {
        let mut resolver = TapResolver::new();
        assert_eq!(resolver.resolve(Input::Down(Key::LeftCommand)), None);
        assert_eq!(resolver.resolve(Input::Other), None);
        assert_eq!(resolver.resolve(Input::Up(Key::LeftCommand)), None);
    }

    #[test]
    fn overlapping_left_and_right_command_does_not_emit() {
        let mut resolver = TapResolver::new();
        assert_eq!(resolver.resolve(Input::Down(Key::LeftCommand)), None);
        assert_eq!(resolver.resolve(Input::Down(Key::RightCommand)), None);
        assert_eq!(resolver.resolve(Input::Up(Key::LeftCommand)), None);
        assert_eq!(resolver.resolve(Input::Up(Key::RightCommand)), None);
    }

    #[test]
    fn consecutive_taps_each_emit() {
        let mut resolver = TapResolver::new();
        assert_eq!(resolver.resolve(Input::Down(Key::LeftCommand)), None);
        assert_eq!(
            resolver.resolve(Input::Up(Key::LeftCommand)),
            Some(Action::Eisu)
        );
        assert_eq!(resolver.resolve(Input::Down(Key::RightCommand)), None);
        assert_eq!(
            resolver.resolve(Input::Up(Key::RightCommand)),
            Some(Action::Kana)
        );
    }

    #[test]
    fn command_up_without_matching_down_does_not_emit() {
        let mut resolver = TapResolver::new();
        assert_eq!(resolver.resolve(Input::Up(Key::LeftCommand)), None);
    }

    #[test]
    fn other_without_held_command_is_noop() {
        let mut resolver = TapResolver::new();
        assert_eq!(resolver.resolve(Input::Other), None);
        assert_eq!(resolver.resolve(Input::Down(Key::LeftCommand)), None);
        assert_eq!(
            resolver.resolve(Input::Up(Key::LeftCommand)),
            Some(Action::Eisu)
        );
    }
}
