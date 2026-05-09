use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
pub enum GameStatVisibility {
    Public,  // Everyone can see this stat.
    Private, // Only the gm can see this stat.
    Hidden, // The stat is hidden from everyone. It can be used for internal calculations and logic.
}

impl Display for GameStatVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameStatVisibility::Public => write!(f, "public"),
            GameStatVisibility::Private => write!(f, "private"),
            GameStatVisibility::Hidden => write!(f, "hidden"),
        }
    }
}

impl Serialize for GameStatVisibility {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for GameStatVisibility {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "public" => Ok(GameStatVisibility::Public),
            "private" => Ok(GameStatVisibility::Private),
            "hidden" => Ok(GameStatVisibility::Hidden),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid GameStatVisibility value: {}",
                s
            ))),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PlayerStatVisibility {
    Public,    // Everyone can see this stat.
    Protected, // Only the player themselves and gm can see this stat.
    Private,   // Only the gm can see this stat.
    Hidden, // The stat is hidden from everyone. It can be used for internal calculations and logic.
}

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

impl Serialize for PlayerStatVisibility {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for PlayerStatVisibility {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "public" => Ok(PlayerStatVisibility::Public),
            "protected" => Ok(PlayerStatVisibility::Protected),
            "private" => Ok(PlayerStatVisibility::Private),
            "hidden" => Ok(PlayerStatVisibility::Hidden),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid PlayerStatVisibility value: {}",
                s
            ))),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ActionVisibility {
    Public,  // Everyone can see and execute this action.
    Private, // Only the gm can see and execute this action.
    Hidden,  // The action is hidden from everyone. It can be used for internal logic.
}

impl Display for ActionVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionVisibility::Public => write!(f, "public"),
            ActionVisibility::Private => write!(f, "private"),
            ActionVisibility::Hidden => write!(f, "hidden"),
        }
    }
}

impl Serialize for ActionVisibility {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for ActionVisibility {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "public" => Ok(ActionVisibility::Public),
            "private" => Ok(ActionVisibility::Private),
            "hidden" => Ok(ActionVisibility::Hidden),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid ActionVisibility value: {}",
                s
            ))),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PageVisibility {
    Public,  // Everyone has the page.
    Private, // Only the gm has the page.
}

impl Display for PageVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PageVisibility::Public => write!(f, "public"),
            PageVisibility::Private => write!(f, "private"),
        }
    }
}

impl Serialize for PageVisibility {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for PageVisibility {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "public" => Ok(PageVisibility::Public),
            "private" => Ok(PageVisibility::Private),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid PageVisibility value: {}",
                s
            ))),
        }
    }
}