#include "piece.h"
#include "texture.h"
#include <SFML/Graphics.hpp>
#include <stdexcept>

Piece::Piece(const sf::Vector2<int> &pos, int faction, int owner, Type type)
{
    this->pos = pos;
    this->faction = faction;
    this->owner = owner;
    this->type = type;
}