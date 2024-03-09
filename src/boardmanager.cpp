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
            tile_buttons[i] = TileButton{sf::FloatRect(offset + sf::Vector2f(tile_size * x, tile_size * y), sf::Vector2f(tile_size, tile_size)), 0, sf::Vector2i(x, y)};
        }
    }

    std::array<Piece::Type, 5> piece_types = {Piece::Type::King, Piece::Type::Queen, Piece::Type::Knight, Piece::Type::Rook, Piece::Type::Bishop};
    for (int i = 0; i < 5; i++)
    {
        promotion_buttons[i] = PromotionButton{sf::FloatRect(0, 0, tile_size, tile_size), 1, {0, 0}, piece_types[i], false};
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

        drawPiece(piece);
    }
}

void BoardManager::drawPiece(const Piece &piece)
{
    sf::Sprite main_sprite(texture.piece_main[piece.type]);
    main_sprite.setScale(scale, scale);
    main_sprite.setColor(colors[piece.faction]);
    main_sprite.setPosition(sf::Vector2f(piece.pos.x * tile_size, piece.pos.y * tile_size) + offset);
    window.draw(main_sprite);

    if (piece.main_owner != -1)
    {
        sf::Sprite base_sprite(texture.piece_base[piece.type]);
        base_sprite.setScale(scale, scale);
        base_sprite.setColor(colors[piece.main_owner]);
        base_sprite.setPosition(sf::Vector2f(piece.pos.x * tile_size, piece.pos.y * tile_size) + offset);
        window.draw(base_sprite);
    }
    else
    {
        sf::Sprite neutral_sprite(texture.piece_neutral[piece.type]);
        neutral_sprite.setScale(scale, scale);
        neutral_sprite.setPosition(sf::Vector2f(piece.pos.x * tile_size, piece.pos.y * tile_size) + offset);
        window.draw(neutral_sprite);
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

    for (int i = 0; i < 5; i++)
    {
        if (!promotion_buttons[i].active)
            continue;
        PromotionButton &promotion_button = promotion_buttons[i];

        drawSquare(promotion_button.pos.x, promotion_button.pos.y + i, sf::Color(255, 255, 255, 255));

        std::array<Piece::Type, 5> piece_types = {Piece::Type::King, Piece::Type::Queen, Piece::Type::Knight, Piece::Type::Rook, Piece::Type::Bishop};

        drawPiece(Piece(promotion_button.pos + sf::Vector2i{0, i}, selected_piece->faction, selected_piece->main_owner, selected_piece->direct_owner, piece_types[i]));
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

        assert(game_states.size() == legal_moves.size());

        Move normal_move = {start_pos, end_pos, piece_moved, is_capture};
        Move promotion_move = {start_pos, end_pos, piece_moved, is_capture, Piece::Type::Queen};

        if (isMoveValid(normal_move))
        {
            GameState game_state = GameState(game_states.back(), normal_move);
            game_states.push_back(std::move(game_state));
            played_moves.push_back(normal_move);
            legal_moves.push_back(game_states.back().getMoves());

            selected_piece = NULL;
        }
        else if (isMoveValid(promotion_move))
        {
            for (int i = 0; i < 5; i++)
            {
                PromotionButton &promotion_button = promotion_buttons[i];
                promotion_button.active = true;
                promotion_button.pos = end_pos;
                promotion_button.rect.left = offset.x + end_pos.x * tile_size;
                promotion_button.rect.top = offset.y + (end_pos.y + i) * tile_size;
            }
        }
        else
        {
            selected_piece = NULL;
            for (int i = 0; i < 5; i++)
            {
                promotion_buttons[i].active = false;
            }
        }
    }
}

void BoardManager::onHold(TileButton &button)
{
}
void BoardManager::onRelease(TileButton &button)
{
}

void BoardManager::onPress(PromotionButton &button)
{
    sf::Vector2i start_pos = selected_piece->pos;
    sf::Vector2i end_pos = getTile(button.pos).pos;
    Piece piece_moved = *selected_piece;
    bool is_capture = getTile(button.pos).piece;

    assert(game_states.size() == legal_moves.size());

    Move move = {start_pos, end_pos, piece_moved, is_capture, button.promotion_type};

    assert(isMoveValid(move));

    GameState game_state = GameState(game_states.back(), move);
    game_states.push_back(std::move(game_state));
    played_moves.push_back(move);
    legal_moves.push_back(game_states.back().getMoves());

    selected_piece = NULL;
    for (int i = 0; i < 5; i++)
    {
        promotion_buttons[i].active = false;
    }
}
void BoardManager::onHold(PromotionButton &button)
{
}
void BoardManager::onRelease(PromotionButton &button)
{
}

void BoardManager::registerListener()
{
    using tfunc = std::function<void(TileButton & button)>;
    void (BoardManager::*top)(TileButton &button) = &onPress;
    void (BoardManager::*toh)(TileButton &button) = &onHold;
    void (BoardManager::*tor)(TileButton &button) = &onRelease;

    tfunc tpress = std::bind(top, this, std::placeholders::_1);
    tfunc thold = std::bind(toh, this, std::placeholders::_1);
    tfunc trelease = std::bind(tor, this, std::placeholders::_1);
    listener_id = ButtonEventChannel<TileButton>::registerListener(tpress, thold, trelease);

    using pfunc = std::function<void(PromotionButton & button)>;
    void (BoardManager::*pop)(PromotionButton &button) = &onPress;
    void (BoardManager::*poh)(PromotionButton &button) = &onHold;
    void (BoardManager::*por)(PromotionButton &button) = &onRelease;

    pfunc ppress = std::bind(pop, this, std::placeholders::_1);
    pfunc phold = std::bind(poh, this, std::placeholders::_1);
    pfunc prelease = std::bind(por, this, std::placeholders::_1);
    listener_id = ButtonEventChannel<PromotionButton>::registerListener(ppress, phold, prelease);
}

void BoardManager::registerButtons(std::vector<Button *> &buttons)
{
    for (auto &button : tile_buttons)
    {
        buttons.push_back(&button);
    }
    for (auto &button : promotion_buttons)
    {
        buttons.push_back(&button);
    }
}

Tile BoardManager::getTile(TileButton &button, int index)
{
    return getTile(button.pos, index);
}

Tile BoardManager::getTile(sf::Vector2i pos, int index)
{
    if (index == -1)
        index = game_states.size() - 1;
    return game_states[index].board[pos.y][pos.x];
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