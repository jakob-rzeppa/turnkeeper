pub enum GameStatVisibility {
    Public,  // Everyone can see this stat.
    Private, // Only the gm can see this stat.
    Hidden, // The stat is hidden from everyone. It can be used for internal calculations and logic.
}

pub enum PlayerStatVisibility {
    Public,    // Everyone can see this stat.
    Protected, // Only the player themselves and gm can see this stat.
    Private,   // Only the gm can see this stat.
    Hidden, // The stat is hidden from everyone. It can be used for internal calculations and logic.
}
