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

    updateSprite();
}

void Piece::updateSprite()
{
    main_sprite.setTexture(texture.piece_main[0]);
    // main_sprite.setTexture(texture.test);
    if (owner == -1)
        base_sprite.setTexture(texture.piece_neutral[type]);
    else
        base_sprite.setTexture(texture.piece_base[type]);
}

void Piece::draw(sf::RenderWindow *window)
{
    window->draw(main_sprite);
    window->draw(base_sprite);
}