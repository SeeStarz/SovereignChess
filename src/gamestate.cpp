#include "piece.h"
#include "gamestate.h"
#include "move.h"
#include "helper.h"
#include "pin.h"
#include <array>
#include <vector>
#include <cassert>
#include <iostream>

GameState::GameState()
{
    // Make sure piece reference won't be dangling
    pieces.reserve(112);
    faction_owner = {0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 11};
    castle_rooks = {true, true, true, true, true, true, true, true};
    player_white_color = 0;
    player_black_color = 11;
    player_white_king = NULL;
    player_black_king = NULL;
    player_white_to_move = true;

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

    makeBoard();
}

GameState::GameState(const GameState &game_state, Move move)
{
    pieces = game_state.pieces;
    faction_owner = game_state.faction_owner;
    castle_rooks = game_state.castle_rooks;
    player_white_color = game_state.player_white_color;
    player_black_color = game_state.player_black_color;
    player_white_to_move = !game_state.player_white_to_move;
    player_white_king = NULL;
    player_black_king = NULL;
    makeBoard();

    Tile &start_tile = board[move.start_pos.y][move.start_pos.x];
    Tile &end_tile = board[move.end_pos.y][move.end_pos.x];
    Piece *piece_moved = start_tile.piece;
    Piece *piece_captured = end_tile.piece;

    assert(piece_moved);
    piece_moved->pos = end_tile.pos;

    start_tile.piece = NULL;
    end_tile.piece = piece_moved;

    if (move.is_capture)
    {
        assert(piece_captured);
        assert(piece_captured->type != Piece::Type::King);
        piece_captured->is_alive = false;

        if (piece_captured->type == Piece::Type::Rook)
        {
            auto it = std::find(castle_rooks_pos.begin(), castle_rooks_pos.end(), piece_captured->pos);
            if (it != castle_rooks_pos.end())
                castle_rooks[it - castle_rooks_pos.begin()] = false;
        }
    }

    if ((piece_moved->type == Piece::Type::King || move.promotion_type == Piece::Type::King) && move.start_pos != move.end_pos)
    {
        if (game_state.player_white_to_move)
            for (int i = 0; i < 4; i++)
                castle_rooks[i] = false;
        else
            for (int i = 4; i < 8; i++)
                castle_rooks[i] = false;
    }
    else if (piece_moved->type == Piece::Type::Rook)
    {
        auto it = std::find(castle_rooks_pos.begin(), castle_rooks_pos.end(), start_tile.pos);
        if (it != castle_rooks_pos.end())
            castle_rooks[it - castle_rooks_pos.begin()] = false;
    }

    if (move.promotion_type != Piece::Type::Pawn && piece_moved->type == Piece::Type::Pawn)
    {
        piece_moved->type = move.promotion_type;
    }

    // Regime Change
    bool update_owner = false;
    if (move.promotion_type == Piece::Type::King)
    {
        update_owner = true;
        Piece *king;
        int start_color;

        // King defection is the only move that modifies faction within Move class
        if (piece_moved->type == Piece::Type::King)
            piece_moved->faction = move.piece_moved.faction;

        if (game_state.player_white_to_move)
        {
            king = player_white_king;
            start_color = player_white_color;
            player_white_king = piece_moved;
            player_white_color = piece_moved->faction;
        }
        else
        {
            king = player_black_king;
            start_color = player_black_color;
            player_black_king = piece_moved;
            player_black_color = piece_moved->faction;
        }

        if (piece_moved != king)
        {
            king->is_alive = false;
            board[king->pos.y][king->pos.x].piece = NULL;
        }

        faction_owner[start_color] = -1;
        faction_owner[piece_moved->faction] = piece_moved->faction;
        auto it = std::find_if(colored_tiles.begin(), colored_tiles.end(), [&start_color](Tile *tile) -> bool
                               { return tile->color == start_color && tile->piece != NULL; });
        if (it != colored_tiles.end())
            faction_owner[start_color] = (*it)->color;
    }

    // Castling
    else if (move.promotion_type == Piece::Type::Rook && piece_moved->type == Piece::Type::King)
    {
        sf::Vector2i direction = (end_tile.pos - start_tile.pos) / (int)(getDistance(end_tile.pos, start_tile.pos) + 0.5);
        assert(direction.y == 0);

        Piece *rook;
        sf::Vector2i rook_pos = start_tile.pos;
        while (true)
        {
            rook_pos += direction;
            assert(checkInBoard(rook_pos));
            Tile &rook_tile = board[rook_pos.y][rook_pos.x];
            if (!rook_tile.piece)
                continue;
            if (rook_tile.piece->type == Piece::Type::Rook)
            {
                rook = rook_tile.piece;
                rook->pos = end_tile.pos - direction;
                board[rook->pos.y][rook->pos.x].piece = rook;
                rook_tile.piece = NULL;
                assert(rook_tile.color == -1);
                break;
            }
            else
                assert(rook_tile.piece->type == Piece::Type::King);
        }
    }

    if (start_tile.color != -1)
    {
        assert(!start_tile.blocked);
        start_tile.other_tile->blocked = false;
        if (start_tile.color != player_white_color && start_tile.color != player_black_color)
            faction_owner[start_tile.color] = -1;
        update_owner = true;
    }
    if (end_tile.color != -1)
    {
        assert(!end_tile.blocked);
        end_tile.other_tile->blocked = true;
        if (end_tile.color != player_white_color && end_tile.color != player_black_color)
            faction_owner[end_tile.color] = piece_moved->faction;
        update_owner = true;
    }

    if (update_owner)
    {
        for (int i = 0; i < pieces.size(); i++)
        {
            Piece &piece = pieces[i];
            if (!piece.is_alive)
                continue;
            piece.direct_owner = faction_owner[piece.faction];
            piece.main_owner = getMainOwner(piece.direct_owner);
        }
    }
}

std::vector<Move> GameState::getMoves()
{
    int ally_color;
    int enemy_color;
    Piece *ally_king;
    if (player_white_to_move)
    {
        ally_color = player_white_color;
        enemy_color = player_black_color;
        ally_king = player_white_king;
    }
    else
    {
        ally_color = player_black_color;
        enemy_color = player_white_color;
        ally_king = player_black_king;
    }

    std::vector<Pin> pins = getAllPins(ally_king->pos, ally_color);

    std::vector<Move> moves;
    for (int i = 0; i < pieces.size(); i++)
    {
        Piece &piece = pieces[i];
        if (!piece.is_alive)
            continue;
        if (piece.main_owner != ally_color)
            continue;

        auto it = std::find_if(pins.begin(), pins.end(), [&piece](Pin p)
                               { return p.pinned_piece->pos == piece.pos; });
        Pin *pin = NULL;
        if (it != pins.end())
            pin = &(*it);

        switch (piece.type)
        {
        case Piece::King:
            getKingMoves(moves, piece);
            break;
        case Piece::Queen:
            getQueenMoves(moves, piece, pin);
            break;
        case Piece::Rook:
            getRookMoves(moves, piece, pin);
            break;
        case Piece::Bishop:
            getBishopMoves(moves, piece, pin);
            break;
        case Piece::Knight:
            getKnightMoves(moves, piece, pin);
            break;
        case Piece::Pawn:
            getPawnMoves(moves, piece, pin);
            break;
        }
    }

    std::vector<Piece *> checking_pieces = getAllChecks(ally_king->pos, ally_color);
    if (checking_pieces.size() != 0)
    {
        std::vector<sf::Vector2i> valid_end_pos = {};

        if (checking_pieces.size() > 1)
        {
            bool same_faction = std::all_of(checking_pieces.begin(), checking_pieces.end(), [&checking_pieces](Piece *piece) -> bool
                                            { return piece->faction == checking_pieces[0]->faction; });

            // It's possible to evade multi check by claiming control
            int faction = checking_pieces[0]->faction;
            bool can_convert = same_faction && faction != enemy_color;
            if (can_convert)
            {
                Tile *tile = NULL;
                for (int i = 0; i < board.size(); i++)
                {
                    auto it = std::find_if(board[i].begin(), board[i].end(), [&faction](Tile &tile) -> bool
                                           { return tile.color == faction && !tile.blocked; });
                    if (it != board[i].end())
                        tile = &(*it);
                }
                assert(tile != NULL);

                valid_end_pos.push_back(tile->pos);
            }
        }
        else
        {
            // Taking control of checking piece
            int faction = checking_pieces[0]->faction;
            bool can_convert = faction != enemy_color;
            if (can_convert)
            {
                auto it = std::find_if(colored_tiles.begin(), colored_tiles.end(), [&faction](Tile *tile) -> bool
                                       { return tile->color == faction && !tile->blocked; });
                assert(it != colored_tiles.end());

                valid_end_pos.push_back((*it)->pos);
            }

            // Capturing checking piece
            valid_end_pos.push_back(checking_pieces[0]->pos);

            // Blocking check
            std::vector<sf::Vector2i> pos_in_between = getInBetweens(ally_king->pos, checking_pieces[0]->pos);
            valid_end_pos.insert(valid_end_pos.end(), pos_in_between.begin(), pos_in_between.end());
        }

        for (int i = moves.size() - 1; i >= 0; i--)
        {
            // King moves already accounts for danger
            bool king_move = moves[i].piece_moved.type == Piece::Type::King;
            bool king_change = moves[i].promotion_type == Piece::Type::King;
            bool part_of_valid_move = std::find(valid_end_pos.begin(), valid_end_pos.end(), moves[i].end_pos) != valid_end_pos.end();
            if (!king_move && !king_change && !part_of_valid_move)
                moves.erase(moves.begin() + i);
        }
    }
    return moves;
}

void GameState::makeBoard()
{
    for (int i = 0; i < board.size(); i++)
    {
        for (int j = 0; j < board[i].size(); j++)
        {
            board[i][j] = Tile{{j, i}, -1, NULL, NULL, false};
        }
    }

    std::array<const sf::Vector2i *, 12> tile_pos = {};
    int j = 0;
    for (auto it = color_map.begin(); it != color_map.end(); it++)
    {
        sf::Vector2i pos = it->first;
        Tile &tile = board[pos.y][pos.x];
        Color color = it->second;

        tile.color = color;
        if (!tile_pos[color])
            tile_pos[color] = &it->first;
        else
        {
            Tile &other_tile = board[tile_pos[color]->y][tile_pos[color]->x];
            tile.other_tile = &other_tile;
            other_tile.other_tile = &tile;
        }

        colored_tiles[j] = &tile;
        j++;
    }

    for (int i = 0; i < pieces.size(); i++)
    {
        Piece &piece = pieces[i];
        if (!piece.is_alive)
            continue;

        Tile &tile = board[piece.pos.y][piece.pos.x];
        tile.piece = &piece;

        assert(!tile.blocked);
        if (tile.color != -1)
            tile.other_tile->blocked = true;

        if (piece.type == Piece::Type::King)
            if (piece.faction == player_white_color)
            {
                assert(player_white_king == NULL);
                player_white_king = &piece;
            }
            else if (piece.faction == player_black_color)
            {
                assert(player_black_king == NULL);
                player_black_king = &piece;
            }
            else
                throw std::runtime_error("Unknown king color " + std::to_string(piece.faction));
    }
}

void GameState::addPiece(int faction, int owner, Piece::Type type, int x, int y)
{
    Piece piece(sf::Vector2i(x, y), faction, owner, owner, type);
    pieces.push_back(piece);
}

int GameState::getMainOwner(int direct_owner)
{
    while (direct_owner != -1 && faction_owner[direct_owner] != direct_owner)
        direct_owner = faction_owner[direct_owner];

    return direct_owner;
}

bool GameState::checkInBoard(sf::Vector2i pos)
{
    if (pos.x < 0 || pos.y < 0)
        return false;
    if (pos.x > 15 || pos.y > 15)
        return false;
    return true;
}

std::vector<Piece *> GameState::getAllChecks(sf::Vector2i pos, int faction)
{
    assert(checkInBoard(pos));
    int tile_color = board[pos.y][pos.x].color;

    int ally_color = getMainOwner(faction);
    int enemy_color;
    if (ally_color == player_white_color)
        enemy_color = player_black_color;
    else
        enemy_color = player_white_color;

    std::vector<Piece *> checking_pieces = {};

    std::array<sf::Vector2i, 8> orthogonal_directions = {{{1, 1}, {1, -1}, {-1, 1}, {-1, -1}, {1, 0}, {0, 1}, {0, -1}, {-1, 0}}};
    // First four are bishops, last four are rooks

    for (int i = 0; i < orthogonal_directions.size(); i++)
    {
        for (int j = 1; j <= 8; j++)
        {
            sf::Vector2i end_pos = pos + orthogonal_directions[i] * j;
            if (!checkInBoard(end_pos))
                break;

            Tile &tile = board[end_pos.y][end_pos.x];
            Piece *target_piece = tile.piece;
            if (target_piece == NULL)
                continue;
            else if (target_piece->main_owner == ally_color && target_piece->type != Piece::Type::King) // Assume the king is moved
                break;
            else if (target_piece->main_owner == -1)
                break;
            else if (target_piece->main_owner == enemy_color)
            {
                // Can't capture on own color
                if (target_piece->faction == tile_color)
                    break;

                if (target_piece->type == Piece::Queen)
                    checking_pieces.push_back(target_piece);
                else if (target_piece->type == Piece::King && j == 1)
                    checking_pieces.push_back(target_piece);
                else if (target_piece->type == Piece::Bishop && i < 4)
                    checking_pieces.push_back(target_piece);
                else if (target_piece->type == Piece::Rook && i >= 4)
                    checking_pieces.push_back(target_piece);
                else if (target_piece->type == Piece::Pawn && j == 1)
                {
                    switch (i)
                    {
                    case 0:
                        if (end_pos.x >= 8 || end_pos.y >= 8)
                            checking_pieces.push_back(target_piece);
                        break;
                    case 1:
                        if (end_pos.x >= 8 || end_pos.y < 8)
                            checking_pieces.push_back(target_piece);
                        break;
                    case 2:
                        if (end_pos.x < 8 || end_pos.y >= 8)
                            checking_pieces.push_back(target_piece);
                        break;
                    case 3:
                        if (end_pos.x < 8 || end_pos.y < 8)
                            checking_pieces.push_back(target_piece);
                        break;
                    }
                }

                // Check is blocked by enemy piece
                break;
            }
        }
    }

    std::array<sf::Vector2i, 8> knight_directions = {{{2, 1}, {2, -1}, {1, 2}, {1, -2}, {-1, 2}, {-1, -2}, {-2, 1}, {-2, -1}}};
    for (int i = 0; i < knight_directions.size(); i++)
    {
        sf::Vector2i end_pos = pos + knight_directions[i];
        if (!checkInBoard(end_pos))
            continue;

        Tile &tile = board[end_pos.y][end_pos.x];
        Piece *target_piece = tile.piece;

        if (target_piece != NULL && target_piece->main_owner == enemy_color && target_piece->type == Piece::Type::Knight && target_piece->faction != tile_color)
            checking_pieces.push_back(target_piece);
    }

    return checking_pieces;
}

std::vector<Pin> GameState::getAllPins(sf::Vector2i pos, int faction)
{
    assert(checkInBoard(pos));
    int tile_color = board[pos.y][pos.x].color;

    int ally_color = getMainOwner(faction);
    int enemy_color;
    if (ally_color == player_white_color)
        enemy_color = player_black_color;
    else
        enemy_color = player_white_color;

    std::vector<Pin> pins = {};

    std::array<sf::Vector2i, 8> directions = {{{1, 1}, {1, -1}, {-1, 1}, {-1, -1}, {1, 0}, {0, 1}, {0, -1}, {-1, 0}}};
    // First four are bishops, last four are rooks

    for (int i = 0; i < directions.size(); i++)
    {
        Piece *pinned_piece = NULL;
        for (int j = 1; j <= 8; j++)
        {
            sf::Vector2i end_pos = pos + directions[i] * j;
            if (!checkInBoard(end_pos))
                break;

            Tile &tile = board[end_pos.y][end_pos.x];
            Piece *target_piece = tile.piece;
            if (target_piece == NULL)
                continue;
            else if (target_piece->main_owner == ally_color && target_piece->type != Piece::Type::King) // Assume the king might be moved
            {
                if (pinned_piece == NULL)
                    pinned_piece = target_piece;
                else
                    break;
            }
            else if (target_piece->main_owner == -1)
                break;
            else if (target_piece->main_owner == enemy_color)
            {
                // If there is no piece in between
                if (!pinned_piece)
                    break;

                // Can't capture on own color
                if (target_piece->faction == tile_color)
                    break;

                if (target_piece->type == Piece::Queen)
                    pins.push_back(Pin{pinned_piece, directions[i], -directions[i]});
                else if (target_piece->type == Piece::Bishop && i < 4)
                    pins.push_back(Pin{pinned_piece, directions[i], -directions[i]});
                else if (target_piece->type == Piece::Rook && i >= 4)
                    pins.push_back(Pin{pinned_piece, directions[i], -directions[i]});

                // Pin is blocked by enemy piece
                break;
            }
        }
    }

    return pins;
}

void GameState::getKingMoves(std::vector<Move> &moves, Piece piece)
{
    int ally_color = piece.main_owner;
    int enemy_color;
    if (piece.main_owner == player_white_color)
        enemy_color = player_black_color;
    else
        enemy_color = player_white_color;

    std::array<sf::Vector2i, 8> directions = {{{1, 1}, {1, 0}, {1, -1}, {0, 1}, {0, -1}, {-1, 1}, {-1, 0}, {-1, -1}}};

    for (int i = 0; i < directions.size(); i++)
    {
        sf::Vector2i end_pos = piece.pos + directions[i];
        if (!checkInBoard(end_pos))
            continue;
        Tile &tile = board[end_pos.y][end_pos.x];
        if (tile.blocked)
            continue;
        if (tile.color == piece.faction)
            continue;
        if (getAllChecks(end_pos, piece.faction).size() != 0)
            continue;

        Piece *target_piece = tile.piece;
        if (target_piece == NULL)
            moves.push_back(Move{piece.pos, end_pos, piece, false});
        else if (target_piece->main_owner == enemy_color)
            moves.push_back(Move{piece.pos, end_pos, piece, true});
    }

    for (int faction = 0; faction < faction_owner.size(); faction++)
    {
        int owner = getMainOwner(faction);

        if (owner == faction)
            continue;
        if (owner != ally_color)
            continue;

        Piece king_piece = piece;
        king_piece.faction = faction;
        if (board[piece.pos.y][piece.pos.x].color != faction)
        {
            if (getAllChecks(piece.pos, faction).size() == 0)
                moves.push_back(Move{piece.pos, piece.pos, king_piece, false, Piece::Type::King});
        }
        else
            for (int j = 0; j < directions.size(); j++)
            {
                sf::Vector2i end_pos = king_piece.pos + directions[j];
                assert(checkInBoard(king_piece.pos));
                Tile &tile = board[end_pos.y][end_pos.x];
                if (tile.blocked)
                    continue;
                if (getAllChecks(end_pos, king_piece.faction).size() != 0)
                    continue;

                Piece *target_piece = tile.piece;
                if (target_piece == NULL)
                    moves.push_back(Move{king_piece.pos, end_pos, king_piece, false, Piece::Type::King});
                else if (target_piece->main_owner == enemy_color)
                    moves.push_back(Move{king_piece.pos, end_pos, king_piece, true, Piece::Type::King});
            }
    }

    for (int i = 0; i < 4; i++)
    {
        if (getAllChecks(piece.pos, piece.faction).size() != 0)
            break;

        int j = player_white_to_move ? j = i : j = i + 4;
        if (!castle_rooks[j])
            continue;

        sf::Vector2i rook_pos = castle_rooks_pos[j];

        assert(board[rook_pos.y][rook_pos.x].piece != NULL);
        if (getMainOwner(board[rook_pos.y][rook_pos.x].piece->faction) != ally_color)
            continue;

        std::vector<sf::Vector2i> in_between = getInBetweens(piece.pos, rook_pos);

        bool empty = true;
        for (sf::Vector2i pos : in_between)
            if (board[pos.y][pos.x].piece != NULL)
            {
                empty = false;
                break;
            }
        if (!empty)
            continue;

        for (sf::Vector2i end_pos : in_between)
            if (getAllChecks(end_pos, piece.faction).size() == 0)
                moves.push_back(Move{piece.pos, end_pos, piece, false, Piece::Type::Rook});
    }
}

void GameState::getQueenMoves(std::vector<Move> &moves, Piece piece, Pin *pin)
{
    getRookMoves(moves, piece, pin);
    getBishopMoves(moves, piece, pin);
}

void GameState::getRookMoves(std::vector<Move> &moves, Piece piece, Pin *pin)
{
    int ally_color = piece.main_owner;
    int enemy_color;
    if (piece.main_owner == player_white_color)
        enemy_color = player_black_color;
    else
        enemy_color = player_white_color;

    std::array<sf::Vector2i, 4> directions = {{{1, 0}, {0, 1}, {0, -1}, {-1, 0}}};

    for (int i = 0; i < directions.size(); i++)
    {
        if (pin != NULL && pin->pin_direction_1 != directions[i] && pin->pin_direction_2 != directions[i])
            continue;

        for (int j = 1; j <= 8; j++)
        {
            sf::Vector2i end_pos = piece.pos + directions[i] * j;
            if (!checkInBoard(end_pos))
                break;

            Tile &tile = board[end_pos.y][end_pos.x];
            if (tile.blocked)
                continue;

            Piece *target_piece = tile.piece;
            if (tile.color == piece.faction)
            {
                if (target_piece == NULL)
                    continue;
                else
                    break;
            }

            if (target_piece == NULL)
                moves.push_back(Move{piece.pos, end_pos, piece, false});
            else if (target_piece->main_owner == enemy_color)
            {
                moves.push_back(Move{piece.pos, end_pos, piece, true});
                break;
            }
            else
                break;
        }
    }
}

void GameState::getBishopMoves(std::vector<Move> &moves, Piece piece, Pin *pin)
{
    int ally_color = piece.main_owner;
    int enemy_color;
    if (piece.main_owner == player_white_color)
        enemy_color = player_black_color;
    else
        enemy_color = player_white_color;

    std::array<sf::Vector2i, 4> directions = {{{1, 1}, {1, -1}, {-1, 1}, {-1, -1}}};

    for (int i = 0; i < directions.size(); i++)
    {
        if (pin != NULL && pin->pin_direction_1 != directions[i] && pin->pin_direction_2 != directions[i])
            continue;

        for (int j = 1; j <= 8; j++)
        {
            sf::Vector2i end_pos = piece.pos + directions[i] * j;
            if (!checkInBoard(end_pos))
                break;

            Tile &tile = board[end_pos.y][end_pos.x];
            if (tile.blocked)
                continue;
            Piece *target_piece = tile.piece;
            if (tile.color == piece.faction)
            {
                if (target_piece == NULL)
                    continue;
                else
                    break;
            }

            if (target_piece == NULL)
                moves.push_back(Move{piece.pos, end_pos, piece, false});
            else if (target_piece->main_owner == enemy_color)
            {
                moves.push_back(Move{piece.pos, end_pos, piece, true});
                break;
            }
            else
                break;
        }
    }
}

void GameState::getKnightMoves(std::vector<Move> &moves, Piece piece, Pin *pin)
{
    if (pin != NULL)
        return;

    int ally_color = piece.main_owner;
    int enemy_color;
    if (piece.main_owner == player_white_color)
        enemy_color = player_black_color;
    else
        enemy_color = player_white_color;

    std::array<sf::Vector2i, 8> directions = {{{2, 1}, {2, -1}, {1, 2}, {1, -2}, {-1, 2}, {-1, -2}, {-2, 1}, {-2, -1}}};

    for (int i = 0; i < directions.size(); i++)
    {
        sf::Vector2i end_pos = piece.pos + directions[i];
        if (!checkInBoard(end_pos))
            continue;

        Tile &tile = board[end_pos.y][end_pos.x];
        if (tile.blocked)
            continue;
        if (tile.color == piece.faction)
            continue;

        Piece *target_piece = tile.piece;
        if (target_piece == NULL)
            moves.push_back(Move{piece.pos, end_pos, piece, false});
        else if (target_piece->main_owner == enemy_color)
            moves.push_back(Move{piece.pos, end_pos, piece, true});
    }
}

void GameState::getPawnMoves(std::vector<Move> &moves, Piece piece, Pin *pin)
{
    int ally_color = piece.main_owner;
    int enemy_color;
    if (piece.main_owner == player_white_color)
        enemy_color = player_black_color;
    else
        enemy_color = player_white_color;

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
        if (pin != NULL && pin->pin_direction_1 != move_directions[i] && pin->pin_direction_2 != move_directions[i])
            continue;

        if (!valid_move_directions[i])
            continue;
        sf::Vector2i end_pos = piece.pos + move_directions[i];
        if (!checkInBoard(end_pos))
            continue;

        Tile &tile = board[end_pos.y][end_pos.x];
        if (tile.blocked)
            continue;
        if (tile.color == piece.faction)
            continue;

        bool promotion = false;
        if (end_pos.x > 5 && end_pos.x < 10 && end_pos.y > 5 && end_pos.y < 10)
            promotion = true;

        Piece *target_piece = tile.piece;
        if (target_piece == NULL)
        {
            if (!promotion)
                moves.push_back(Move{piece.pos, end_pos, piece, false});
            else
            {
                moves.push_back(Move{piece.pos, end_pos, piece, false, Piece::Type::Queen});
                moves.push_back(Move{piece.pos, end_pos, piece, false, Piece::Type::Rook});
                moves.push_back(Move{piece.pos, end_pos, piece, false, Piece::Type::Bishop});
                moves.push_back(Move{piece.pos, end_pos, piece, false, Piece::Type::Knight});

                Tile &tile = board[piece.pos.y][piece.pos.x];
                Piece *pointed_piece = tile.piece;
                assert(tile.piece != NULL);
                tile.piece = NULL;
                assert(pointed_piece->is_alive);
                pointed_piece->is_alive = false;

                if (getAllChecks(end_pos, piece.faction).size() == 0)
                    moves.push_back(Move{piece.pos, end_pos, piece, false, Piece::Type::King});

                pointed_piece->is_alive = true;
                tile.piece = pointed_piece;
            }

            // Double move
            if (!valid_double_directions[i])
                continue;
            end_pos = piece.pos + move_directions[i] * 2;
            target_piece = board[end_pos.y][end_pos.x].piece;
            if (target_piece == NULL)
                moves.push_back(Move{piece.pos, end_pos, piece, false});
        }
    }

    for (int i = 0; i < capture_directions.size(); i++)
    {
        if (pin != NULL && pin->pin_direction_1 != capture_directions[i] && pin->pin_direction_2 != capture_directions[i])
            continue;

        if (!valid_capture_directions[i])
            continue;
        sf::Vector2i end_pos = piece.pos + capture_directions[i];
        if (!checkInBoard(end_pos))
            continue;

        Tile &tile = board[end_pos.y][end_pos.x];
        if (tile.blocked)
            continue;
        if (tile.color == piece.faction)
            continue;

        Piece *target_piece = tile.piece;
        if (target_piece && target_piece->main_owner == enemy_color)
            moves.push_back(Move{piece.pos, end_pos, piece, true});
    }
}