#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Use an efficient WASM allocator.
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

use alloy_primitives::{Address, U8};
use chess_engine::{Board, BoardBuilder, Color, GameResult, Move, Piece, Position};

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{alloy_primitives::U256, console, msg, prelude::*};

// game status
const CONTINUING: u8 = 1;
const ILLEGAL_MOVE: u8 = 2;
const STALEMATE: u8 = 3;
const VICTORY: u8 = 4;

// colors
const WHITE: u8 = 0;
const BLACK: u8 = 1;

/// Piece types
const PAWN: u8 = 1;
const KNIGHT: u8 = 2;
const BISHOP: u8 = 3;
const ROOK: u8 = 4;
const QUEEN: u8 = 5;
const KING: u8 = 6;

/// Bit masks
const COLOR_MASK: u8 = 1;
const PIECE_TYPE_MASK: u8 = 7;

sol_storage! {
    #[entrypoint]
    pub struct OnchainChess{
      /// Total games of chess started
      uint256 total_games;
      /// Used to store a single pending game while waiting for a player two to join.
      uint256 pending_game;
      /// Stores info for each chess game
      mapping(uint256 => GameInfo) games;
    }

    pub struct GameInfo{
    /// Player 1 is WHITE
    address player_one;
    /// Player 2 is BLACK
    address player_two;
    /// PENDING (waiting second player) = 0, CONTINUING = 1, STALEMATE = 3, or VICTORY = 4
    uint8 game_status;
    /// Player turn 0 = WHITE; 1 = BLACK
    uint8 turn_color;
    /// 0 = WHITE; 1 = BLACK
    uint8 victor;
    /// All the info needed to rebuild the board
    uint256 board_state
    }
}
