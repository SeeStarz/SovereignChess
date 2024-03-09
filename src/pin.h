#ifndef PIN_H
#define PIN_H

#include "piece.h"
#include <SFML/System/Vector2.hpp>

struct Pin
{
    Piece *pinned_piece;
    sf::Vector2i pin_direction_1;
    sf::Vector2i pin_direction_2;
};

#endif