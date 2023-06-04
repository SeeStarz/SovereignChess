#include "piece.h"
#include "gamestate.h"
#include <array>
#include <vector>

GameState::GameState(Texture &texture) : texture(texture)
{
    player1_color = 0;
    player2_color = 11;
    player1_to_move = true;
    {
        addPiece(0, 0, Piece::King, 8, 15);
        addPiece(11, 11, Piece::King, 8, 0);
        addPiece(0, 0, Piece::Queen, 7, 15);
        addPiece(11, 11, Piece::Queen, 7, 0);
        addPiece(1, -1, Piece::Queen, 0, 7);
        addPiece(2, -1, Piece::Queen, 0, 0);
        addPiece(2, -1, Piece::Queen, 15, 15);
        addPiece(3, -1, Piece::Queen, 15, 12);
        addPiece(4, -1, Piece::Queen, 0, 3);
        addPiece(5, -1, Piece::Queen, 15, 8);
        addPiece(6, -1, Piece::Queen, 15, 7);
        addPiece(7, -1, Piece::Queen, 0, 12);
        addPiece(8, -1, Piece::Queen, 15, 3);
        addPiece(9, -1, Piece::Queen, 0, 15);
        addPiece(9, -1, Piece::Queen, 15, 0);
        addPiece(10, -1, Piece::Queen, 0, 8);
        addPiece(0, 0, Piece::Rook, 4, 15);
        addPiece(0, 0, Piece::Rook, 11, 15);
        addPiece(11, 11, Piece::Rook, 4, 0);
        addPiece(11, 11, Piece::Rook, 11, 0);
        addPiece(1, -1, Piece::Rook, 13, 15);
        addPiece(2, -1, Piece::Rook, 15, 14);
        addPiece(2, -1, Piece::Rook, 0, 1);
        addPiece(3, -1, Piece::Rook, 0, 4);
        addPiece(4, -1, Piece::Rook, 15, 11);
        addPiece(5, -1, Piece::Rook, 2, 0);
        addPiece(6, -1, Piece::Rook, 2, 15);
        addPiece(7, -1, Piece::Rook, 15, 4);
        addPiece(8, -1, Piece::Rook, 0, 11);
        addPiece(9, -1, Piece::Rook, 0, 14);
        addPiece(9, -1, Piece::Rook, 15, 1);
        addPiece(10, -1, Piece::Rook, 13, 0);
        addPiece(0, 0, Piece::Bishop, 6, 15);
        addPiece(0, 0, Piece::Bishop, 9, 15);
        addPiece(11, 11, Piece::Bishop, 6, 0);
        addPiece(11, 11, Piece::Bishop, 9, 0);
        addPiece(1, -1, Piece::Bishop, 0, 6);
        addPiece(2, -1, Piece::Bishop, 1, 0);
        addPiece(2, -1, Piece::Bishop, 14, 15);
        addPiece(3, -1, Piece::Bishop, 15, 13);
        addPiece(4, -1, Piece::Bishop, 0, 2);
        addPiece(5, -1, Piece::Bishop, 15, 9);
        addPiece(6, -1, Piece::Bishop, 15, 6);
        addPiece(7, -1, Piece::Bishop, 0, 13);
        addPiece(8, -1, Piece::Bishop, 15, 2);
        addPiece(9, -1, Piece::Bishop, 1, 15);
        addPiece(9, -1, Piece::Bishop, 14, 0);
        addPiece(10, -1, Piece::Bishop, 0, 9);
        addPiece(0, 0, Piece::Knight, 5, 15);
        addPiece(0, 0, Piece::Knight, 10, 15);
        addPiece(11, 11, Piece::Knight, 5, 0);
        addPiece(11, 11, Piece::Knight, 10, 0);
        addPiece(1, -1, Piece::Knight, 12, 15);
        addPiece(2, -1, Piece::Knight, 14, 14);
        addPiece(2, -1, Piece::Knight, 1, 1);
        addPiece(3, -1, Piece::Knight, 0, 5);
        addPiece(4, -1, Piece::Knight, 15, 10);
        addPiece(5, -1, Piece::Knight, 3, 0);
        addPiece(6, -1, Piece::Knight, 3, 15);
        addPiece(7, -1, Piece::Knight, 15, 5);
        addPiece(8, -1, Piece::Knight, 0, 10);
        addPiece(9, -1, Piece::Knight, 14, 1);
        addPiece(9, -1, Piece::Knight, 1, 14);
        addPiece(10, -1, Piece::Knight, 12, 0);
        addPiece(0, 0, Piece::Pawn, 4, 14);
        addPiece(0, 0, Piece::Pawn, 5, 14);
        addPiece(0, 0, Piece::Pawn, 6, 14);
        addPiece(0, 0, Piece::Pawn, 7, 14);
        addPiece(0, 0, Piece::Pawn, 8, 14);
        addPiece(0, 0, Piece::Pawn, 9, 14);
        addPiece(0, 0, Piece::Pawn, 10, 14);
        addPiece(0, 0, Piece::Pawn, 11, 14);
        addPiece(11, 11, Piece::Pawn, 4, 1);
        addPiece(11, 11, Piece::Pawn, 5, 1);
        addPiece(11, 11, Piece::Pawn, 6, 1);
        addPiece(11, 11, Piece::Pawn, 7, 1);
        addPiece(11, 11, Piece::Pawn, 8, 1);
        addPiece(11, 11, Piece::Pawn, 9, 1);
        addPiece(11, 11, Piece::Pawn, 10, 1);
        addPiece(11, 11, Piece::Pawn, 11, 1);
        addPiece(1, -1, Piece::Pawn, 12, 14);
        addPiece(1, -1, Piece::Pawn, 13, 14);
        addPiece(3, -1, Piece::Pawn, 14, 13);
        addPiece(3, -1, Piece::Pawn, 14, 12);
        addPiece(4, -1, Piece::Pawn, 14, 11);
        addPiece(4, -1, Piece::Pawn, 14, 10);
        addPiece(5, -1, Piece::Pawn, 14, 9);
        addPiece(5, -1, Piece::Pawn, 14, 8);
        addPiece(6, -1, Piece::Pawn, 14, 7);
        addPiece(6, -1, Piece::Pawn, 14, 6);
        addPiece(7, -1, Piece::Pawn, 14, 5);
        addPiece(7, -1, Piece::Pawn, 14, 4);
        addPiece(8, -1, Piece::Pawn, 14, 3);
        addPiece(8, -1, Piece::Pawn, 14, 2);
        addPiece(10, -1, Piece::Pawn, 13, 1);
        addPiece(10, -1, Piece::Pawn, 12, 1);
        addPiece(5, -1, Piece::Pawn, 3, 1);
        addPiece(5, -1, Piece::Pawn, 2, 1);
        addPiece(4, -1, Piece::Pawn, 1, 2);
        addPiece(4, -1, Piece::Pawn, 1, 3);
        addPiece(3, -1, Piece::Pawn, 1, 4);
        addPiece(3, -1, Piece::Pawn, 1, 5);
        addPiece(1, -1, Piece::Pawn, 1, 6);
        addPiece(1, -1, Piece::Pawn, 1, 7);
        addPiece(10, -1, Piece::Pawn, 1, 8);
        addPiece(10, -1, Piece::Pawn, 1, 9);
        addPiece(8, -1, Piece::Pawn, 1, 10);
        addPiece(8, -1, Piece::Pawn, 1, 11);
        addPiece(7, -1, Piece::Pawn, 1, 12);
        addPiece(7, -1, Piece::Pawn, 1, 13);
        addPiece(6, -1, Piece::Pawn, 2, 14);
        addPiece(6, -1, Piece::Pawn, 3, 14);
    }
}

void GameState::addPiece(int faction, int owner, Piece::Type type, int r, int c)
{
    Piece piece(sf::Vector2i(r, c), faction, owner, type, texture);
    board[r][c] = &piece;
    pieces.push_back(piece);
}