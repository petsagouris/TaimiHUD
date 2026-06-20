use bitflags::bitflags;

#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    strum::EnumString,
    strum::Display,
    strum::AsRefStr,
    strum::IntoStaticStr,
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[strum(serialize_all = "lowercase", ascii_case_insensitive)]
pub enum Festival {
    Halloween,
    Wintersday,
    #[cfg_attr(feature = "serde", serde(rename = "superadventurefestival"))]
    #[strum(serialize = "superadventurefestival")]
    SuperAdventureBox,
    LunarNewYear,
    #[cfg_attr(feature = "serde", serde(rename = "festivalofthefourwinds"))]
    #[strum(serialize = "festivalofthefourwinds")]
    FourWinds,
    DragonBash,
}

impl Festival {
    pub const ALL: &'static [Self] = &[
        Self::LunarNewYear,
        Self::SuperAdventureBox,
        Self::DragonBash,
        Self::FourWinds,
        Self::Halloween,
        Self::Wintersday,
    ];

    pub fn all() -> impl Iterator<Item = Self> + Clone {
        Self::ALL.iter().copied()
    }

    pub fn as_str(&self) -> &'static str {
        // via strum::IntoStaticStr
        self.into()
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Festivals: u16 {
        const LUNAR_NEW_YEAR = 0x01;
        const SUPER_ADVENTURE_BOX = 0x02;
        const DRAGON_BASH = 0x04;
        const FOUR_WINDS = 0x08;
        const HALLOWEEN = 0x10;
        const WINTERSDAY = 0x20;
    }
}

impl Festivals {
    pub const fn for_festival(festival: Festival) -> Self {
        match festival {
            Festival::LunarNewYear => Self::LUNAR_NEW_YEAR,
            Festival::SuperAdventureBox => Self::SUPER_ADVENTURE_BOX,
            Festival::DragonBash => Self::DRAGON_BASH,
            Festival::FourWinds => Self::FOUR_WINDS,
            Festival::Halloween => Self::HALLOWEEN,
            Festival::Wintersday => Self::WINTERSDAY,
        }
    }

    pub const fn to_festival(self) -> Option<Festival> {
        Some(match self {
            Self::LUNAR_NEW_YEAR => Festival::LunarNewYear,
            Self::SUPER_ADVENTURE_BOX => Festival::SuperAdventureBox,
            Self::DRAGON_BASH => Festival::DragonBash,
            Self::FOUR_WINDS => Festival::FourWinds,
            Self::HALLOWEEN => Festival::Halloween,
            Self::WINTERSDAY => Festival::Wintersday,
            _ => return None,
        })
    }

    pub const fn get(&self, festival: Festival) -> bool {
        self.contains(Self::for_festival(festival))
    }

    pub fn iter_festivals(self) -> impl Iterator<Item = Festival> {
        self.into_iter().filter_map(Self::to_festival)
    }
}

impl From<Festival> for Festivals {
    fn from(festival: Festival) -> Self {
        Self::for_festival(festival)
    }
}
impl From<Option<Festival>> for Festivals {
    fn from(festival: Option<Festival>) -> Self {
        festival.map(Into::into).unwrap_or(Self::empty())
    }
}
impl FromIterator<Festival> for Festivals {
    fn from_iter<T: IntoIterator<Item = Festival>>(iter: T) -> Self {
        iter.into_iter().map(Self::from).collect()
    }
}
impl<'a> FromIterator<&'a Festival> for Festivals {
    fn from_iter<T: IntoIterator<Item = &'a Festival>>(iter: T) -> Self {
        iter.into_iter().map(|&f| Self::from(f)).collect()
    }
}
