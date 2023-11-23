#include "move.h"
#include <SFML/System/Vector2.hpp>

bool Move::operator==(const Move &other)
{
    if (start_pos != other.start_pos)
        return false;
    if (end_pos != other.end_pos)
        return false;
    if (piece_moved != other.piece_moved)
        return false;
    if (is_capture != other.is_capture)
        return false;
    if (promotion_type != other.promotion_type)
        return false;
    return true;
}

bool Move::operator!=(const Move &other)
{
    return !(*this == other);
}