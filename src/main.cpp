#include "piece.h"
#include "texture.h"
#include <SFML/Graphics.hpp>
#include <iostream>
#include <vector>

int main()
{
    sf::RenderWindow window(sf::VideoMode(1600, 900), "Test For SFML");
    Texture texture;
    texture.load();

    std::vector<Piece> pieces;
    pieces.push_back(Piece(sf::Vector2i(1, 1), 11, 11, Piece::Type::Rook, texture));
    pieces[0].base_sprite.setScale(10, 10);
    pieces[0].main_sprite.setScale(10, 10);

    while (window.isOpen())
    {
        sf::Event event;
        while (window.pollEvent(event))
        {
            if (event.type == sf::Event::Closed)
                window.close();

            if (sf::Keyboard::isKeyPressed(sf::Keyboard::W))
            {
                throw std::runtime_error("amogus");
            }
        }

        window.clear(sf::Color(255, 255, 255));
        for (int i = 0; i < pieces.size(); i++)
        {
            window.draw(pieces[i].main_sprite);
            window.draw(pieces[i].base_sprite);
        }
        window.display();
    }

    return 0;
}