#include "piece.h"
#include "texture.h"
#include <SFML/Graphics.hpp>
#include <stdexcept>

Piece::Piece(sf::Vector2<int> pos, int faction, int owner, Type type, Texture &texture) : texture(texture)
{
    this->pos = pos;
    this->faction = faction;
    this->owner = owner;
    this->type = type;

    colors[0] = sf::Color(255, 255, 255);
    colors[1] = sf::Color(31, 159, 31);
    colors[2] = sf::Color(191, 191, 191);
    colors[3] = sf::Color(31, 191, 191);
    colors[4] = sf::Color(31, 31, 191);
    colors[5] = sf::Color(159, 0, 223);
    colors[6] = sf::Color(223, 159, 223);
    colors[7] = sf::Color(191, 31, 31);
    colors[8] = sf::Color(223, 95, 31);
    colors[9] = sf::Color(95, 95, 95);
    colors[10] = sf::Color(255, 255, 31);
    colors[11] = sf::Color(31, 31, 31);

    updateSprite();
}

void Piece::updateSprite()
{
    main_sprite.setTexture(texture.piece_main[0]);
    if (owner == -1)
        base_sprite.setTexture(texture.piece_neutral[type]);
    else
        base_sprite.setTexture(texture.piece_base[type]);

    main_sprite.setColor(colors[faction]);
    if (owner == -1)
        base_sprite.setColor(colors[0]);
    else
        base_sprite.setColor(colors[owner]);
}

void Piece::draw(sf::RenderWindow *window)
{
    window->draw(main_sprite);
    window->draw(base_sprite);
}