#ifndef MOVE_H
#define MOVE_H

#include "piece.h"
#include <SFML/System/Vector2.hpp>
struct Move
{
public:
    sf::Vector2i start_pos;
    sf::Vector2i end_pos;
    Piece piece_moved;
    bool is_capture;
};

#endif