#include "piece.h"
#include "texture.h"
#include <SFML/Graphics.hpp>
#include <stdexcept>

Piece::Piece(const sf::Vector2<int> &pos, int faction, int main_owner, int direct_owner, Type type)
{
    this->pos = pos;
    this->faction = faction;
    this->main_owner = main_owner;
    this->direct_owner = direct_owner;
    this->type = type;
}

bool Piece::operator==(const Piece &other)
{
    if (pos != other.pos)
        return false;
    if (faction != other.faction)
        return false;
    if (main_owner != other.main_owner)
        return false;
    if (direct_owner != other.direct_owner)
        return false;
    if (type != other.type)
        return false;
    return true;
}

bool Piece::operator!=(const Piece &other)
{
    return !(*this == other);
}