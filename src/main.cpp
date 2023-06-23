#include "piece.h"
#include "texture.h"
#include "constants.cpp"
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
    sf::RectangleShape square = sf::RectangleShape(sf::Vector2f(48, 48));
    square.setFillColor(color);
    square.setPosition(48 * x, 48 * y);
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

    {
        drawSquare(window, 4, 4, colors[Red]);
        drawSquare(window, 11, 11, colors[Red]);
        drawSquare(window, 11, 4, colors[Blue]);
        drawSquare(window, 4, 11, colors[Blue]);
        drawSquare(window, 5, 5, colors[Yellow]);
        drawSquare(window, 10, 10, colors[Yellow]);
        drawSquare(window, 10, 5, colors[Green]);
        drawSquare(window, 5, 10, colors[Green]);
        drawSquare(window, 7, 5, colors[Pink]);
        drawSquare(window, 8, 10, colors[Pink]);
        drawSquare(window, 8, 5, colors[Purple]);
        drawSquare(window, 7, 10, colors[Purple]);
        drawSquare(window, 6, 6, colors[DarkGray]);
        drawSquare(window, 9, 9, colors[DarkGray]);
        drawSquare(window, 9, 6, colors[LightGray]);
        drawSquare(window, 6, 9, colors[LightGray]);
        drawSquare(window, 5, 7, colors[Orange]);
        drawSquare(window, 10, 8, colors[Orange]);
        drawSquare(window, 10, 7, colors[Cyan]);
        drawSquare(window, 5, 8, colors[Cyan]);
        drawSquare(window, 7, 7, colors[White]);
        drawSquare(window, 8, 8, colors[White]);
        drawSquare(window, 8, 7, colors[Black]);
        drawSquare(window, 7, 8, colors[Black]);
    }

    sf::RectangleShape horizontal_line(sf::Vector2f(768, 2));
    sf::RectangleShape vertical_line(sf::Vector2f(2, 768));
    horizontal_line.setFillColor(sf::Color::Black);
    vertical_line.setFillColor(sf::Color::Black);
    horizontal_line.setOrigin(1, 1);
    vertical_line.setOrigin(1, 1);
    for (int i = 0; i <= 16; i++)
    {
        horizontal_line.setPosition(0, 48 * i);
        vertical_line.setPosition(48 * i, 0);
        window.draw(horizontal_line);
        window.draw(vertical_line);
    }

    sf::RectangleShape board_border(sf::Vector2f(768, 768));
    board_border.setFillColor(sf::Color::Transparent);
    board_border.setOutlineColor(sf::Color::Black);
    board_border.setOutlineThickness(5);
    window.draw(board_border);

    sf::RectangleShape horizontal_pawn(sf::Vector2f(288, 4));
    horizontal_pawn.setOrigin(0, 2);
    horizontal_pawn.setFillColor(sf::Color(66, 47, 43));
    horizontal_pawn.setPosition(0, 384);
    window.draw(horizontal_pawn);
    horizontal_pawn.setPosition(480, 384);
    window.draw(horizontal_pawn);

    sf::RectangleShape vertical_pawn(sf::Vector2f(4, 288));
    vertical_pawn.setOrigin(2, 0);
    vertical_pawn.setFillColor(sf::Color(66, 47, 43));
    vertical_pawn.setPosition(384, 0);
    window.draw(vertical_pawn);
    vertical_pawn.setPosition(384, 480);
    window.draw(vertical_pawn);

    sf::RectangleShape promotion_border(sf::Vector2f(192, 192));
    promotion_border.setPosition(288, 288);
    promotion_border.setFillColor(sf::Color::Transparent);
    promotion_border.setOutlineColor(sf::Color::Black);
    promotion_border.setOutlineThickness(3);
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
        main_sprite.setPosition(piece.pos.x * 48, piece.pos.y * 48);
        window.draw(main_sprite);

        sf::Sprite base_sprite(texture.piece_base[piece.type]);
        base_sprite.setScale(3, 3);
        base_sprite.setColor(colors[piece.owner]);
        base_sprite.setPosition(piece.pos.x * 48, piece.pos.y * 48);
        window.draw(base_sprite);
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
    sf::RenderWindow window(sf::VideoMode(1600, 900), "Test For SFML");
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
                    sf::Vector2i board_pos = mouse_pos / 48;
                    if (board_pos.x >= 0 && board_pos.x < 16 && board_pos.y >= 0 && board_pos.y < 16)
                        selected_piece = game_state.board[board_pos.y][board_pos.x];
                }
                else
                {
                    bool valid = false;
                    sf::Vector2i mouse_pos = sf::Mouse::getPosition(window);
                    sf::Vector2i board_pos = mouse_pos / 48;
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
                        selected_piece = game_state.board[board_pos.y][board_pos.x];
                }
            }
        }

        draw(window, texture, game_state.pieces, selected_piece, moves);
    }

    return 0;
}