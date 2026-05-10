use std::{ fmt::Display, str::FromStr };

use backend_derive::{ deserialize_use_from_str, serialize_use_display };

#[derive(Clone, Debug, PartialEq)]
pub enum GameStatVisibility {
    Public, // Everyone can see this stat.
    Private, // Only the gm can see this stat.
    Hidden, // The stat is hidden from everyone. It can be used for internal calculations and logic.
}

#[serialize_use_display]
impl Display for GameStatVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameStatVisibility::Public => write!(f, "public"),
            GameStatVisibility::Private => write!(f, "private"),
            GameStatVisibility::Hidden => write!(f, "hidden"),
        }
    }
}

#[deserialize_use_from_str]
impl FromStr for GameStatVisibility {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "public" => Ok(GameStatVisibility::Public),
            "private" => Ok(GameStatVisibility::Private),
            "hidden" => Ok(GameStatVisibility::Hidden),
            _ => Err(format!("Invalid GameStatVisibility value: {}", s)),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PlayerStatVisibility {
    Public, // Everyone can see this stat.
    Protected, // Only the player themselves and gm can see this stat.
    Private, // Only the gm can see this stat.
    Hidden, // The stat is hidden from everyone. It can be used for internal calculations and logic.
}

#[serialize_use_display]
impl Display for PlayerStatVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerStatVisibility::Public => write!(f, "public"),
            PlayerStatVisibility::Protected => write!(f, "protected"),
            PlayerStatVisibility::Private => write!(f, "private"),
            PlayerStatVisibility::Hidden => write!(f, "hidden"),
        }
    }
}

#[deserialize_use_from_str]
impl FromStr for PlayerStatVisibility {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "public" => Ok(PlayerStatVisibility::Public),
            "protected" => Ok(PlayerStatVisibility::Protected),
            "private" => Ok(PlayerStatVisibility::Private),
            "hidden" => Ok(PlayerStatVisibility::Hidden),
            _ => Err(format!("Invalid PlayerStatVisibility value: {}", s)),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ActionVisibility {
    Public, // Everyone can see and execute this action.
    Private, // Only the gm can see and execute this action.
    Hidden, // The action is hidden from everyone. It can be used for internal logic.
}

#[serialize_use_display]
impl Display for ActionVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionVisibility::Public => write!(f, "public"),
            ActionVisibility::Private => write!(f, "private"),
            ActionVisibility::Hidden => write!(f, "hidden"),
        }
    }
}

#[deserialize_use_from_str]
impl FromStr for ActionVisibility {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "public" => Ok(ActionVisibility::Public),
            "private" => Ok(ActionVisibility::Private),
            "hidden" => Ok(ActionVisibility::Hidden),
            _ => Err(format!("Invalid ActionVisibility value: {}", s)),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PageVisibility {
    Public, // Everyone has the page.
    Private, // Only the gm has the page.
}

#[serialize_use_display]
impl Display for PageVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PageVisibility::Public => write!(f, "public"),
            PageVisibility::Private => write!(f, "private"),
        }
    }
}

#[deserialize_use_from_str]
impl FromStr for PageVisibility {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "public" => Ok(PageVisibility::Public),
            "private" => Ok(PageVisibility::Private),
            _ => Err(format!("Invalid PageVisibility value: {}", s)),
        }
    }
}
