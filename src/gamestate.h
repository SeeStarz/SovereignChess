#ifndef GAMESTATE_H
#define GAMESTATE_H

#include "piece.h"
#include "move.h"
#include "tile.h"
#include "pin.h"
#include <array>
#include <vector>
#include <map>

class GameState
{
private:
    void makeBoard();
    void addPiece(int faction, int owner, Piece::Type type, int r, int c);

    void getKingMoves(std::vector<Move> &moves, Piece piece);
    void getQueenMoves(std::vector<Move> &moves, Piece piece, Pin *pin);
    void getRookMoves(std::vector<Move> &moves, Piece piece, Pin *pin);
    void getBishopMoves(std::vector<Move> &moves, Piece piece, Pin *pin);
    void getKnightMoves(std::vector<Move> &moves, Piece piece, Pin *pin);
    void getPawnMoves(std::vector<Move> &moves, Piece piece, Pin *pin);

public:
    std::array<std::array<Tile, 16>, 16> board;
    std::array<Tile *, 24> colored_tiles;
    std::vector<Piece> pieces;
    std::array<int, 12> faction_owner;
    std::array<bool, 8> castle_rooks; // From left to right, first 4 are white's
    std::vector<std::array<sf::Vector2i, 2>> pins;
    Piece *player_white_king;
    Piece *player_black_king;
    int player_white_color;
    int player_black_color;
    bool player_white_to_move;
    // Player white/black refers to the player's starting color

    GameState();
    GameState(const GameState &game_state, Move move);
    std::vector<Move> getMoves();

    int getMainOwner(int direct_owner);
    static bool checkInBoard(sf::Vector2i pos);
    std::vector<Piece *> getAllChecks(sf::Vector2i pos, int faction);
    std::vector<Pin> getAllPins(sf::Vector2i pos, int faction);

    // GameState is only moveable
    GameState(const GameState &) = delete;
    GameState &operator=(const GameState &) = delete;
    GameState(GameState &&) = default;
    GameState &operator=(GameState &&) = default;
};

#endif