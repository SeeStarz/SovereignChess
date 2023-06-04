#include "piece.h"
#include "texture.h"
#include "constants.cpp"
#include <SFML/Graphics.hpp>
#include <stdexcept>

Piece::Piece(const sf::Vector2<int> &pos, int faction, int owner, Type type, Texture &texture) : texture(texture)
{
    this->pos = pos;
    this->faction = faction;
    this->owner = owner;
    this->type = type;

    updateSprite();
}

void Piece::updateSprite()
{
    main_sprite.setTexture(texture.piece_main[type]);
    if (owner == -1)
        base_sprite.setTexture(texture.piece_neutral[type]);
    else
        base_sprite.setTexture(texture.piece_base[type]);

    main_sprite.setColor(colors[faction]);
    if (owner == -1)
        base_sprite.setColor(colors[owner]);
    else
        base_sprite.setColor(colors[owner]);
}