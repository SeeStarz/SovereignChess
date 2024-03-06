#ifndef GAMESTATE_H
#define GAMESTATE_H

#include "piece.h"
#include "move.h"
#include "tile.h"
#include <array>
#include <vector>
#include <map>

class GameState
{
private:
    void makeBoard();
    void addPiece(int faction, int owner, Piece::Type type, int r, int c);
    int getMainOwner(int direct_owner);
    bool checkInBoard(sf::Vector2i pos);
    bool checkIsCheck(sf::Vector2i pos, int faction);

    void getKingMoves(std::vector<Move> &moves, Piece piece);
    void getQueenMoves(std::vector<Move> &moves, Piece piece);
    void getRookMoves(std::vector<Move> &moves, Piece piece);
    void getBishopMoves(std::vector<Move> &moves, Piece piece);
    void getKnightMoves(std::vector<Move> &moves, Piece piece);
    void getPawnMoves(std::vector<Move> &moves, Piece piece);

public:
    std::array<std::array<Tile, 16>, 16> board;
    std::vector<Piece> pieces;
    std::array<int, 12> faction_owner;
    std::vector<std::array<sf::Vector2i, 2>> pins;
    int player1_color;
    int player2_color;
    bool player1_to_move;

    GameState();
    GameState(const GameState &game_state, Move move);
    std::vector<Move> getMoves();

    // GameState is only moveable
    GameState(const GameState &) = delete;
    GameState &operator=(const GameState &) = delete;
    GameState(GameState &&) = default;
    GameState &operator=(GameState &&) = default;
};

#endif