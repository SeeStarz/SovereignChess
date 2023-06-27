#include "piece.h"
#include "texture.h"
#include "helper.h"
#include "displayconfig.h"
#include "gamestate.h"
#include "move.h"
#include <SFML/Graphics.hpp>
#include <memory>
#include <iostream>
#include <vector>
#include <array>
#include <map>

void drawSquare(sf::RenderWindow &window, int x, int y, sf::Color color)
{
    sf::RectangleShape square = sf::RectangleShape(sf::Vector2f(end_size, end_size));
    square.setFillColor(color);
    square.setPosition(sf::Vector2f(end_size * x, end_size * y) + offset);
    window.draw(square);
}

void drawBoard(sf::RenderWindow &window)
{
    for (int i = 0; i < 16; i++)
    {
        for (int j = 0; j < 16; j++)
        {
            if ((i + j) % 2 == 0)
                drawSquare(window, i, j, sf::Color(255, 248, 220));
            else
                drawSquare(window, i, j, sf::Color(222, 184, 135));
        }
    }

    for (auto it = color_map.begin(); it != color_map.end(); it++)
        drawSquare(window, it->first.x, it->first.y, colors[it->second]);

    sf::RectangleShape horizontal_line(sf::Vector2f(end_size * 16, line_scale * 2));
    sf::RectangleShape vertical_line(sf::Vector2f(line_scale * 2, end_size * 16));
    horizontal_line.setFillColor(sf::Color::Black);
    vertical_line.setFillColor(sf::Color::Black);
    horizontal_line.setOrigin(line_scale, line_scale);
    vertical_line.setOrigin(line_scale, line_scale);
    for (int i = 0; i <= 16; i++)
    {
        horizontal_line.setPosition(sf::Vector2f(0, end_size * i) + offset);
        vertical_line.setPosition(sf::Vector2f(end_size * i, 0) + offset);
        window.draw(horizontal_line);
        window.draw(vertical_line);
    }

    sf::RectangleShape board_border(sf::Vector2f(end_size * 16, end_size * 16));
    board_border.setPosition(offset);
    board_border.setFillColor(sf::Color::Transparent);
    board_border.setOutlineColor(sf::Color::Black);
    board_border.setOutlineThickness(line_scale * 5);
    window.draw(board_border);

    sf::RectangleShape horizontal_pawn(sf::Vector2f(end_size * 6, line_scale * 4));
    horizontal_pawn.setOrigin(0, line_scale * 2);
    horizontal_pawn.setFillColor(sf::Color(66, 47, 43));
    horizontal_pawn.setPosition(sf::Vector2f(0, end_size * 8) + offset);
    window.draw(horizontal_pawn);
    horizontal_pawn.setPosition(sf::Vector2f(end_size * 10, end_size * 8) + offset);
    window.draw(horizontal_pawn);

    sf::RectangleShape vertical_pawn(sf::Vector2f(line_scale * 4, end_size * 6));
    vertical_pawn.setOrigin(line_scale * 2, 0);
    vertical_pawn.setFillColor(sf::Color(66, 47, 43));
    vertical_pawn.setPosition(sf::Vector2f(end_size * 8, 0) + offset);
    window.draw(vertical_pawn);
    vertical_pawn.setPosition(sf::Vector2f(end_size * 8, end_size * 10) + offset);
    window.draw(vertical_pawn);

    sf::RectangleShape promotion_border(sf::Vector2f(end_size * 4, end_size * 4));
    promotion_border.setPosition(sf::Vector2f(end_size * 6, end_size * 6) + offset);
    promotion_border.setFillColor(sf::Color::Transparent);
    promotion_border.setOutlineColor(sf::Color::Black);
    promotion_border.setOutlineThickness(line_scale * 3);
    window.draw(promotion_border);
}

void drawPieces(sf::RenderWindow &window, Texture &texture, std::vector<Piece> &pieces)
{
    for (int i = 0; i < pieces.size(); i++)
    {
        Piece &piece = pieces[i];
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

void drawMoves(sf::RenderWindow &window, Piece *piece, std::vector<Move> moves)
{
    if (!piece)
        return;
    drawSquare(window, piece->pos.x, piece->pos.y, sf::Color(0, 255, 0, 50));
    for (int i = 0; i < moves.size(); i++)
    {
        if (moves[i].start_position != piece->pos)
            continue;

        drawSquare(window, moves[i].end_position.x, moves[i].end_position.y, sf::Color(255, 0, 0, 50));
    }
}

void draw(sf::RenderWindow &window, Texture &texture, std::vector<Piece> &pieces, Piece *piece, std::vector<Move> moves)
{
    window.clear(sf::Color(255, 255, 255));
    drawBoard(window);
    drawPieces(window, texture, pieces);
    drawMoves(window, piece, moves);
    window.display();
}

int main()
{
    sf::RenderWindow window(sf::VideoMode(width, height), "Test For SFML");
    Texture texture;
    texture.load();

    std::vector<Move> moves;
    Piece *selected_piece = NULL;

    GameState game_state;
    moves = game_state.getMoves();

    while (window.isOpen())
    {
        sf::Event event;
        while (window.pollEvent(event))
        {
            if (event.type == sf::Event::Closed)
                window.close();
            if (event.type == sf::Event::MouseButtonPressed)
            {
                if (!selected_piece)
                {
                    sf::Vector2i mouse_pos = sf::Mouse::getPosition(window);
                    sf::Vector2i board_pos = mouse_pos / (int)end_size;
                    if (board_pos.x >= 0 && board_pos.x < 16 && board_pos.y >= 0 && board_pos.y < 16)
                        selected_piece = game_state.board[board_pos.y][board_pos.x].piece;
                }
                else
                {
                    bool valid = false;
                    sf::Vector2i mouse_pos = sf::Mouse::getPosition(window);
                    sf::Vector2i board_pos = mouse_pos / (int)end_size;
                    for (int i = 0; i < moves.size(); i++)
                    {
                        Move &move = moves[i];
                        if (board_pos != move.end_position)
                            continue;
                        if (selected_piece->pos != move.start_position)
                            continue;

                        valid = true;
                        game_state = GameState(game_state, move);
                        moves = game_state.getMoves();
                        selected_piece = NULL;
                        break;
                    }
                    if (!valid && board_pos.x >= 0 && board_pos.x < 16 && board_pos.y >= 0 && board_pos.y < 16)
                        selected_piece = game_state.board[board_pos.y][board_pos.x].piece;
                }
            }
        }

        draw(window, texture, game_state.pieces, selected_piece, moves);
    }

    return 0;
}