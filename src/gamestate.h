#ifndef GAMESTATE_H
#define GAMESTATE_H

#include "piece.h"
#include "move.h"
#include <array>
#include <vector>

class GameState
{
private:
    void getKingMoves(std::vector<Move> &moves, Piece piece);
    void getQueenMoves(std::vector<Move> &moves, Piece piece);
    void getRookMoves(std::vector<Move> &moves, Piece piece);
    void getBishopMoves(std::vector<Move> &moves, Piece piece);
    void getKnightMoves(std::vector<Move> &moves, Piece piece);
    void getPawnMoves(std::vector<Move> &moves, Piece piece);

    bool checkInBoard(sf::Vector2i pos);

public:
    std::array<std::array<Piece *, 16>, 16> board;
    std::vector<Piece> pieces;
    int player1_color;
    int player2_color;
    bool player1_to_move;

    GameState();
    GameState(const GameState &game_state, Move move);
    std::vector<Move> getMoves();

    void addPiece(int faction, int owner, Piece::Type type, int r, int c);
};

#endif