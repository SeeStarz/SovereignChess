#ifndef GAMESTATE_H
#define GAMESTATE_H

#include "piece.h"
#include <array>
#include <vector>

class GameState
{
public:
    std::array<std::array<Piece *, 16>, 16> board;
    std::vector<Piece> pieces;
    int player1_color;
    int player2_color;
    bool player1_to_move;
    Texture &texture;

    GameState(Texture &texture);

    void addPiece(int faction, int owner, Piece::Type type, int r, int c);
};

#endif