use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum GameStatVisibility {
    Public,  // Everyone can see this stat.
    Private, // Only the gm can see this stat.
    Hidden, // The stat is hidden from everyone. It can be used for internal calculations and logic.
}

impl Display for GameStatVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameStatVisibility::Public => write!(f, "Public"),
            GameStatVisibility::Private => write!(f, "Private"),
            GameStatVisibility::Hidden => write!(f, "Hidden"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum PlayerStatVisibility {
    Public,    // Everyone can see this stat.
    Protected, // Only the player themselves and gm can see this stat.
    Private,   // Only the gm can see this stat.
    Hidden, // The stat is hidden from everyone. It can be used for internal calculations and logic.
}

impl Display for PlayerStatVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerStatVisibility::Public => write!(f, "Public"),
            PlayerStatVisibility::Protected => write!(f, "Protected"),
            PlayerStatVisibility::Private => write!(f, "Private"),
            PlayerStatVisibility::Hidden => write!(f, "Hidden"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ActionVisibility {
    Public,  // Everyone can see and execute this action.
    Private, // Only the gm can see and execute this action.
    Hidden,  // The action is hidden from everyone. It can be used for internal logic.
}

impl Display for ActionVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionVisibility::Public => write!(f, "Public"),
            ActionVisibility::Private => write!(f, "Private"),
            ActionVisibility::Hidden => write!(f, "Hidden"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum PageVisibility {
    Public,  // Everyone has the page.
    Private, // Only the gm has the page.
}

impl Display for PageVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PageVisibility::Public => write!(f, "Public"),
            PageVisibility::Private => write!(f, "Private"),
        }
    }
}
