pub mod repository;
pub mod error;

pub mod command {
    pub mod game;
}

pub mod entity {
    pub mod game;
    pub mod user;
    pub mod player;
    pub mod stat;
}

pub mod value_object {
    pub mod name;
    pub mod password;
    pub mod stat {
        pub mod key;
        pub mod boolean_value;
        pub mod number_value;
        pub mod string_value;
    }
}