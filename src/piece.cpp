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