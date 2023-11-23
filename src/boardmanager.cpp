#include "boardmanager.h"
#include "helper.h"
#include "displayconfig.h"
#include "piece.h"
#include "move.h"
#include "event.h"
#include "button.h"
#include <SFML/Graphics.hpp>
#include <utility>
#include <cassert>
#include <iostream>
#include <functional>

BoardManager::BoardManager(sf::RenderWindow &window) : window(window)
{
    game_states.push_back(GameState());
    played_moves = {};
    legal_moves.push_back(game_states[0].getMoves());
    texture = Texture();
    texture.load();
    selected_piece = NULL;
    registerListener();

    for (int x = 0; x < 16; x++)
    {
        for (int y = 0; y < 16; y++)
        {
            int i = 16 * x + y;
            tile_buttons[i] = TileButton{sf::FloatRect(offset + sf::Vector2f(tile_size * x, tile_size * y), sf::Vector2f(tile_size, tile_size)), sf::Vector2i(x, y), 0};
        }
    }
}

void BoardManager::drawSquare(int x, int y, sf::Color color)
{
    sf::RectangleShape square = sf::RectangleShape(sf::Vector2f(tile_size, tile_size));
    square.setFillColor(color);
    square.setPosition(sf::Vector2f(tile_size * x, tile_size * y) + offset);
    window.draw(square);
}

void BoardManager::drawBoard()
{
    for (int i = 0; i < 16; i++)
    {
        for (int j = 0; j < 16; j++)
        {
            if ((i + j) % 2 == 0)
                drawSquare(i, j, sf::Color(255, 248, 220));
            else
                drawSquare(i, j, sf::Color(222, 184, 135));
        }
    }

    for (auto it = color_map.begin(); it != color_map.end(); it++)
        drawSquare(it->first.x, it->first.y, colors[it->second]);

    sf::RectangleShape horizontal_line(sf::Vector2f(tile_size * 16, line_scale * 2));
    sf::RectangleShape vertical_line(sf::Vector2f(line_scale * 2, tile_size * 16));
    horizontal_line.setFillColor(sf::Color::Black);
    vertical_line.setFillColor(sf::Color::Black);
    horizontal_line.setOrigin(line_scale, line_scale);
    vertical_line.setOrigin(line_scale, line_scale);
    for (int i = 0; i <= 16; i++)
    {
        horizontal_line.setPosition(sf::Vector2f(0, tile_size * i) + offset);
        vertical_line.setPosition(sf::Vector2f(tile_size * i, 0) + offset);
        window.draw(horizontal_line);
        window.draw(vertical_line);
    }

    sf::RectangleShape board_border(sf::Vector2f(tile_size * 16, tile_size * 16));
    board_border.setPosition(offset);
    board_border.setFillColor(sf::Color::Transparent);
    board_border.setOutlineColor(sf::Color::Black);
    board_border.setOutlineThickness(line_scale * 5);
    window.draw(board_border);

    sf::RectangleShape horizontal_pawn(sf::Vector2f(tile_size * 6, line_scale * 4));
    horizontal_pawn.setOrigin(0, line_scale * 2);
    horizontal_pawn.setFillColor(sf::Color(66, 47, 43));
    horizontal_pawn.setPosition(sf::Vector2f(0, tile_size * 8) + offset);
    window.draw(horizontal_pawn);
    horizontal_pawn.setPosition(sf::Vector2f(tile_size * 10, tile_size * 8) + offset);
    window.draw(horizontal_pawn);

    sf::RectangleShape vertical_pawn(sf::Vector2f(line_scale * 4, tile_size * 6));
    vertical_pawn.setOrigin(line_scale * 2, 0);
    vertical_pawn.setFillColor(sf::Color(66, 47, 43));
    vertical_pawn.setPosition(sf::Vector2f(tile_size * 8, 0) + offset);
    window.draw(vertical_pawn);
    vertical_pawn.setPosition(sf::Vector2f(tile_size * 8, tile_size * 10) + offset);
    window.draw(vertical_pawn);

    sf::RectangleShape promotion_border(sf::Vector2f(tile_size * 4, tile_size * 4));
    promotion_border.setPosition(sf::Vector2f(tile_size * 6, tile_size * 6) + offset);
    promotion_border.setFillColor(sf::Color::Transparent);
    promotion_border.setOutlineColor(sf::Color::Black);
    promotion_border.setOutlineThickness(line_scale * 3);
    window.draw(promotion_border);
}

void BoardManager::drawPieces()
{
    const std::vector<Piece> &pieces = game_states.back().pieces;
    for (int i = 0; i < pieces.size(); i++)
    {
        const Piece &piece = pieces[i];
        if (!piece.is_alive)
            continue;

        sf::Sprite main_sprite(texture.piece_main[piece.type]);
        main_sprite.setScale(3, 3);
        main_sprite.setColor(colors[piece.faction]);
        main_sprite.setPosition(sf::Vector2f(piece.pos.x * 48, piece.pos.y * 48) + offset);
        window.draw(main_sprite);

        if (piece.main_owner != -1)
        {
            sf::Sprite base_sprite(texture.piece_base[piece.type]);
            base_sprite.setScale(3, 3);
            base_sprite.setColor(colors[piece.main_owner]);
            base_sprite.setPosition(sf::Vector2f(piece.pos.x * 48, piece.pos.y * 48) + offset);
            window.draw(base_sprite);
        }
        else
        {
            sf::Sprite neutral_sprite(texture.piece_neutral[piece.type]);
            neutral_sprite.setScale(3, 3);
            neutral_sprite.setPosition(sf::Vector2f(piece.pos.x * 48, piece.pos.y * 48) + offset);
            window.draw(neutral_sprite);
        }
    }
}

void BoardManager::drawMoves()
{
    assert(legal_moves.size() == game_states.size());
    std::vector<Move> moves = legal_moves.back();
    if (!selected_piece)
        return;
    drawSquare(selected_piece->pos.x, selected_piece->pos.y, sf::Color(0, 255, 0, 50));
    for (int i = 0; i < moves.size(); i++)
    {
        if (moves[i].start_pos != selected_piece->pos)
            continue;

        drawSquare(moves[i].end_pos.x, moves[i].end_pos.y, sf::Color(255, 0, 0, 50));
    }
}

void BoardManager::draw()
{
    drawBoard();
    drawPieces();
    drawMoves();
}

void BoardManager::onPress(TileButton &button)
{
    if (!selected_piece)
        selected_piece = getTile(button).piece;
    else
    {
        sf::Vector2i start_pos = selected_piece->pos;
        sf::Vector2i end_pos = getTile(button).pos;
        Piece piece_moved = *selected_piece;
        bool is_capture = getTile(button).piece;
        Move move = {start_pos, end_pos, piece_moved, is_capture};
        Move move2 = {start_pos, end_pos, piece_moved, is_capture, Piece::Type::Queen};

        assert(game_states.size() == legal_moves.size());
        if (isMoveValid(move))
        {
            GameState game_state = GameState(game_states.back(), move);
            game_states.push_back(std::move(game_state));
            played_moves.push_back(move);
            legal_moves.push_back(game_states.back().getMoves());
        }
        if (isMoveValid(move2))
        {
            GameState game_state = GameState(game_states.back(), move2);
            game_states.push_back(std::move(game_state));
            played_moves.push_back(move2);
            legal_moves.push_back(game_states.back().getMoves());
        }

        selected_piece = NULL;
    }
}
void BoardManager::onHold(TileButton &button)
{
}
void BoardManager::onRelease(TileButton &button)
{
}

void BoardManager::registerListener()
{
    using func = std::function<void(TileButton & button)>;
    func press = std::bind(&onPress, this, std::placeholders::_1);
    func hold = std::bind(&onHold, this, std::placeholders::_1);
    func release = std::bind(&onRelease, this, std::placeholders::_1);
    listener_id = ButtonEventChannel<TileButton>::registerListener(press, hold, release);
}

void BoardManager::registerButtons(std::vector<Button *> &buttons)
{
    for (auto &button : tile_buttons)
    {
        buttons.push_back(&button);
    }
}

Tile BoardManager::getTile(TileButton &button, int index)
{
    if (index == -1)
        index = game_states.size() - 1;
    return game_states[index].board[button.pos.y][button.pos.x];
}

bool BoardManager::isMoveValid(const Move &move, int index)
{
    if (index == -1)
        index = legal_moves.size() - 1;
    for (Move legal_move : legal_moves[index])
        if (legal_move == move)
            return true;
    return false;
}