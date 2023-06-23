#include "piece.h"
#include "gamestate.h"
#include "move.h"
#include <array>
#include <vector>
#include <cassert>
#include <iostream>

GameState::GameState()
{
    // Make sure piece reference won't be dangling
    pieces.reserve(256);
    player1_color = 0;
    player2_color = 11;
    player1_to_move = true;
    for (int i = 0; i < board.size(); i++)
    {
        for (int j = 0; j < board[i].size(); j++)
            board[i][j] = NULL;
    }
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

GameState::GameState(const GameState &game_state, Move move)
{
    pieces = game_state.pieces;
    player1_color = game_state.player1_color;
    player2_color = game_state.player2_color;
    player1_to_move = !game_state.player1_to_move;

    for (int i = 0; i < board.size(); i++)
    {
        for (int j = 0; j < board[i].size(); j++)
        {
            board[i][j] = NULL;
        }
    }

    for (int i = 0; i < pieces.size(); i++)
    {
        Piece &piece = pieces[i];
        if (!piece.is_alive)
            continue;
        board[piece.pos.y][piece.pos.x] = &piece;
    }

    Piece *piece_moved = board[move.start_position.y][move.start_position.x];
    Piece *piece_captured = board[move.end_position.y][move.end_position.x];

    assert(piece_moved != NULL);
    piece_moved->pos = move.end_position;
    if (move.is_capture)
    {
        assert(piece_captured != NULL);
        piece_captured->is_alive = false;
    }

    board[move.start_position.y][move.start_position.x] = NULL;
    board[move.end_position.y][move.end_position.x] = piece_moved;
}

std::vector<Move> GameState::getMoves()
{
    int controlled;
    if (player1_to_move)
        controlled = player1_color;
    else
        controlled = player2_color;

    std::vector<Move> moves;
    for (int i = 0; i < pieces.size(); i++)
    {
        Piece &piece = pieces[i];
        if (!piece.is_alive)
            continue;
        if (piece.owner != controlled)
            continue;

        switch (piece.type)
        {
        case Piece::King:
            getKingMoves(moves, piece);
            break;
        case Piece::Queen:
            getQueenMoves(moves, piece);
            break;
        case Piece::Rook:
            getRookMoves(moves, piece);
            break;
        case Piece::Bishop:
            getBishopMoves(moves, piece);
            break;
        case Piece::Knight:
            getKnightMoves(moves, piece);
            break;
        case Piece::Pawn:
            getPawnMoves(moves, piece);
            break;
        }
    }

    return moves;
}

void GameState::addPiece(int faction, int owner, Piece::Type type, int x, int y)
{
    Piece piece(sf::Vector2i(x, y), faction, owner, type);
    pieces.push_back(piece);
    board[y][x] = &pieces.back();
}

void GameState::getKingMoves(std::vector<Move> &moves, Piece piece)
{
    int ally_color = piece.owner;
    int enemy_color;
    if (piece.owner == player1_color)
        enemy_color = player2_color;
    else
        enemy_color = player1_color;

    std::array<sf::Vector2i, 8> directions = {{{1, 1}, {1, 0}, {1, -1}, {0, 1}, {0, -1}, {-1, 1}, {-1, 0}, {-1, -1}}};

    for (int i = 0; i < directions.size(); i++)
    {
        sf::Vector2i end_pos = piece.pos + directions[i];
        if (!checkInBoard(end_pos))
            continue;

        Piece *target_piece = board[end_pos.y][end_pos.x];
        if (target_piece == NULL)
            moves.push_back(Move{piece.pos, end_pos, piece, false});
        else if (target_piece->owner == enemy_color)
            moves.push_back(Move{piece.pos, end_pos, piece, true});
    }
}
void GameState::getQueenMoves(std::vector<Move> &moves, Piece piece)
{
    getRookMoves(moves, piece);
    getBishopMoves(moves, piece);
}
void GameState::getRookMoves(std::vector<Move> &moves, Piece piece)
{
    int ally_color = piece.owner;
    int enemy_color;
    if (piece.owner == player1_color)
        enemy_color = player2_color;
    else
        enemy_color = player1_color;

    std::array<sf::Vector2i, 4> directions = {{{1, 0}, {0, 1}, {0, -1}, {-1, 0}}};

    for (int i = 0; i < directions.size(); i++)
    {
        for (int j = 1; j <= 8; j++)
        {
            sf::Vector2i end_pos = piece.pos + directions[i] * j;
            if (!checkInBoard(end_pos))
                break;

            Piece *target_piece = board[end_pos.y][end_pos.x];
            if (target_piece == NULL)
                moves.push_back(Move{piece.pos, end_pos, piece, false});
            else if (target_piece->owner == enemy_color)
            {
                moves.push_back(Move{piece.pos, end_pos, piece, true});
                break;
            }
            else
                break;
        }
    }
}
void GameState::getBishopMoves(std::vector<Move> &moves, Piece piece)
{
    {
        int ally_color = piece.owner;
        int enemy_color;
        if (piece.owner == player1_color)
            enemy_color = player2_color;
        else
            enemy_color = player1_color;

        std::array<sf::Vector2i, 4> directions = {{{1, 1}, {1, -1}, {-1, 1}, {-1, -1}}};

        for (int i = 0; i < directions.size(); i++)
        {
            for (int j = 1; j <= 8; j++)
            {
                sf::Vector2i end_pos = piece.pos + directions[i] * j;
                if (!checkInBoard(end_pos))
                    break;

                Piece *target_piece = board[end_pos.y][end_pos.x];
                if (target_piece == NULL)
                    moves.push_back(Move{piece.pos, end_pos, piece, false});
                else if (target_piece->owner == enemy_color)
                {
                    moves.push_back(Move{piece.pos, end_pos, piece, true});
                    break;
                }
                else
                    break;
            }
        }
    }
}
void GameState::getKnightMoves(std::vector<Move> &moves, Piece piece)
{
    int ally_color = piece.owner;
    int enemy_color;
    if (piece.owner == player1_color)
        enemy_color = player2_color;
    else
        enemy_color = player1_color;

    std::array<sf::Vector2i, 8> directions = {{{2, 1}, {2, -1}, {1, 2}, {1, -2}, {-1, 2}, {-1, -2}, {-2, 1}, {-2, -1}}};

    for (int i = 0; i < directions.size(); i++)
    {
        sf::Vector2i end_pos = piece.pos + directions[i];
        if (!checkInBoard(end_pos))
            continue;

        Piece *target_piece = board[end_pos.y][end_pos.x];
        if (target_piece == NULL)
            moves.push_back(Move{piece.pos, end_pos, piece, false});
        else if (target_piece->owner == enemy_color)
            moves.push_back(Move{piece.pos, end_pos, piece, true});
    }
}
void GameState::getPawnMoves(std::vector<Move> &moves, Piece piece)
{
    int ally_color = piece.owner;
    int enemy_color;
    if (piece.owner == player1_color)
        enemy_color = player2_color;
    else
        enemy_color = player1_color;

    std::array<sf::Vector2i, 4> move_directions = {{{1, 0}, {0, 1}, {0, -1}, {-1, 0}}};
    std::array<sf::Vector2i, 4> capture_directions = {{{1, 1}, {1, -1}, {-1, 1}, {-1, -1}}};
    std::array<bool, 4> valid_move_directions = {};
    std::array<bool, 4> valid_double_directions = {};
    std::array<bool, 4> valid_capture_directions = {};

    if (piece.pos.x < 7)
    {
        valid_move_directions[0] = true;
        valid_capture_directions[0] = true;
        valid_capture_directions[1] = true;

        if (piece.pos.x == 0)
            valid_double_directions[0] = true;
        else if (piece.pos.x == 1 && piece.pos.y != 0 && piece.pos.y != 15)
            valid_double_directions[0] = true;
    }

    if (piece.pos.x > 8)
    {
        valid_move_directions[3] = true;
        valid_capture_directions[2] = true;
        valid_capture_directions[3] = true;

        if (piece.pos.x == 15)
            valid_double_directions[3] = true;
        else if (piece.pos.x == 14 && piece.pos.y != 0 && piece.pos.y != 15)
            valid_double_directions[3] = true;
    }
    if (piece.pos.y < 7)
    {
        valid_move_directions[1] = true;
        valid_capture_directions[0] = true;
        valid_capture_directions[2] = true;

        if (piece.pos.y == 0)
            valid_double_directions[1] = true;
        else if (piece.pos.y == 1 && piece.pos.x != 0 && piece.pos.x != 15)
            valid_double_directions[1] = true;
    }

    if (piece.pos.y > 8)
    {
        valid_move_directions[2] = true;
        valid_capture_directions[1] = true;
        valid_capture_directions[3] = true;

        if (piece.pos.y == 15)
            valid_double_directions[2] = true;
        else if (piece.pos.y == 14 && piece.pos.x != 0 && piece.pos.x != 15)
            valid_double_directions[2] = true;
    }

    for (int i = 0; i < move_directions.size(); i++)
    {
        if (!valid_move_directions[i])
            continue;
        sf::Vector2i end_pos = piece.pos + move_directions[i];
        if (!checkInBoard(end_pos))
            continue;

        Piece *target_piece = board[end_pos.y][end_pos.x];
        if (target_piece == NULL)
        {
            moves.push_back(Move{piece.pos, end_pos, piece, false});

            // Double move
            if (!valid_double_directions[i])
                continue;
            sf::Vector2i end_pos = piece.pos + move_directions[i] * 2;
            Piece *target_piece = board[end_pos.y][end_pos.x];
            if (target_piece == NULL)
                moves.push_back(Move{piece.pos, end_pos, piece, false});
        }
    }

    for (int i = 0; i < capture_directions.size(); i++)
    {
        if (!valid_capture_directions[i])
            continue;
        sf::Vector2i end_pos = piece.pos + capture_directions[i];
        if (!checkInBoard(end_pos))
            continue;
        Piece *target_piece = board[end_pos.y][end_pos.x];
        if (target_piece && target_piece->owner == enemy_color)
            moves.push_back(Move{piece.pos, end_pos, piece, true});
    }
}

bool GameState::checkInBoard(sf::Vector2i pos)
{
    if (pos.x < 0 || pos.y < 0)
        return false;
    if (pos.x > 15 || pos.y > 15)
        return false;
    return true;
}