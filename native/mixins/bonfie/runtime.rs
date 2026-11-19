#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BonfieMode {
    Follow = 0,
    Guard = 1,
    Wander = 2,
    Caretaker = 3,
    Gather = 4,
    Treasure = 5,
}

impl BonfieMode {
    pub fn next(self) -> Self {
        match self {
            Self::Follow => Self::Guard,
            Self::Guard => Self::Wander,
            Self::Wander => Self::Caretaker,
            Self::Caretaker => Self::Gather,
            Self::Gather => Self::Treasure,
            Self::Treasure => Self::Follow,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FollowPolicy {
    Passive = 0,
    HostileOnly = 1,
    All = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkKind {
    None = 0,
    Chop = 1,
    Mine = 2,
    Fish = 3,
    Flower = 4,
    Brew = 5,
    Sort = 6,
    Cook = 7,
}

#[derive(Debug, Clone, Copy)]
pub struct BonfieState {
    pub mode: BonfieMode,
    pub follow_policy: FollowPolicy,
    pub mood: f32,
    pub affection: f32,
    pub work: WorkKind,
    pub dancing: bool,
    pub sitting_until: u64,
    pub lying_until: u64,
    pub last_interaction: u64,
}

impl Default for BonfieState {
    fn default() -> Self {
        Self {
            mode: BonfieMode::Follow,
            follow_policy: FollowPolicy::Passive,
            mood: 75.0,
            affection: 50.0,
            work: WorkKind::None,
            dancing: false,
            sitting_until: 0,
            lying_until: 0,
            last_interaction: 0,
        }
    }
}

impl BonfieState {
    pub fn positive_interaction(&mut self, now: u64, mood: f32, affection: f32) {
        self.mood = (self.mood + mood).clamp(0.0, 100.0);
        self.affection = (self.affection + affection).clamp(0.0, 100.0);
        self.last_interaction = now;
    }

    pub fn tick(&mut self, now: u64, raining: bool, thundering: bool) {
        if now % 100 == 0 && now.saturating_sub(self.last_interaction) > 600 {
            self.mood = (self.mood - 0.02).max(0.0);
            self.affection = (self.affection - 0.01).max(0.0);
        }
        if raining {
            self.mood = (self.mood - 0.001).max(0.0);
        }
        if thundering {
            self.mood = (self.mood - 0.003).max(0.0);
        }
        if now >= self.sitting_until {
            self.sitting_until = 0;
        }
        if now >= self.lying_until {
            self.lying_until = 0;
        }
    }

    pub fn work_efficiency(&self) -> f32 {
        if self.mood >= 75.0 { 1.20 }
        else if self.mood <= 25.0 { 0.70 }
        else { 1.0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mode_cycle_matches_original_six_modes() {
        let mut mode = BonfieMode::Follow;
        for expected in [
            BonfieMode::Guard,
            BonfieMode::Wander,
            BonfieMode::Caretaker,
            BonfieMode::Gather,
            BonfieMode::Treasure,
            BonfieMode::Follow,
        ] {
            mode = mode.next();
            assert_eq!(mode, expected);
        }
    }
}
