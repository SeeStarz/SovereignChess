#ifndef TILE_H
#define TILE_H

#include "piece.h"
#include <SFML/System/Vector2.hpp>
#include <memory>

class Tile
{
public:
    sf::Vector2i pos;
    int color;
    std::unique_ptr<Piece> piece;
};

#endif