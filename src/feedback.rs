#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cue {
    Coin,
    Jump,
    Shot,
    Hit,
    Hurt,
    Win,
    Lose,
    Portal,
    Count,
    Warning,
    Phase,
    Block,
    Dodge,
    Punch,
}
impl Cue {
    pub fn file(self) -> &'static str {
        match self {
            Self::Coin => "coin",
            Self::Jump => "jump",
            Self::Shot => "shot",
            Self::Hit => "hit",
            Self::Hurt => "hurt",
            Self::Win => "win",
            Self::Lose => "lose",
            Self::Portal => "portal",
            Self::Count => "count",
            Self::Warning => "warning",
            Self::Phase => "phase",
            Self::Block => "block",
            Self::Dodge => "dodge",
            Self::Punch => "punch",
        }
    }
}
