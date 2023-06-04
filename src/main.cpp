#include "piece.h"
#include "texture.h"
#include <SFML/Graphics.hpp>
#include <iostream>

int main()
{
    sf::RenderWindow window(sf::VideoMode(1600, 900), "Test For SFML");
    Texture texture;
    texture.load();

    // sf::Sprite f;
    // f.setTexture(texture.piece_main[4]);
    // f.setScale(10, 10);

    Piece piece(sf::Vector2i(1, 1), 0, 0, Piece::Type::Rook, texture);
    piece.main_sprite.setScale(10, 10);
    piece.base_sprite.setScale(10, 10);

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
        window.draw(piece.main_sprite);
        window.draw(piece.base_sprite);
        // window.draw(f);
        window.display();
    }

    return 0;
}