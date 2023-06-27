#ifndef TILE_H
#define TILE_H

#include "piece.h"
#include <SFML/System/Vector2.hpp>
#include <memory>

struct Tile
{
    sf::Vector2i position;
    int color;
    Piece *piece;
    Tile *other_tile;
    bool blocked;
};

#endif