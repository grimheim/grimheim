//! State
//!
//! This module acts as a state machine for the service

use std::convert::Into;

/// Selecting a lobby
struct LobbySelect;

/// Creating a new game
struct CreateGame;

/// In a lobby
struct Lobby;

/// In a game
struct GameSession;

/// In a post game screen
struct PostGame;

impl Default for LobbySelect {
    fn default() -> Self {
        LobbySelect { }
    }
}

impl Into<CreateGame> for LobbySelect {
    fn into(self) -> CreateGame {
        CreateGame { }
    }
}

impl Into<Lobby> for LobbySelect {
    fn into(self) -> Lobby {
        Lobby { }
    }
}

impl Into<GameSession> for Lobby {
    fn into(self) -> GameSession {
        GameSession { }
    }
}

impl Into<PostGame> for GameSession {
    fn into(self) -> PostGame {
        PostGame { }
    }
}

impl Into<Lobby> for PostGame {
    fn into(self) -> Lobby {
        Lobby { }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lobby_select_default() {
        LobbySelect::default();
    }

    #[test]
    fn lobby_select_into_create_game() {
        let _: CreateGame = LobbySelect{}.into();
    }

    #[test]
    fn lobby_select_into_lobby() {
        let _: Lobby = LobbySelect{}.into();
    }

    #[test]
    fn lobby_into_game_session() {
        let _: GameSession = Lobby{}.into();
    }

    #[test]
    fn game_session_into_post_game() {
        let _: PostGame = GameSession{}.into();
    }

    #[test]
    fn post_game_into_lobby() {
        let _: Lobby = PostGame{}.into();
    }
}
