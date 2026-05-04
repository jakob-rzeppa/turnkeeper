pub mod error;
pub mod lexer;
pub mod macros;
pub mod parsable;

pub mod parsables {
    pub mod atoms {
        pub mod variable_type;
    }

    pub mod roots {
        pub mod stat;
        pub mod player_stat;
        pub mod action;
    }
}
