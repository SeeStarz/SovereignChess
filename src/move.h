#ifndef MOVE_H
#define MOVE_H

#include "piece.h"
#include <SFML/System/Vector2.hpp>
#include <SFML/Network/Packet.hpp>
#include <array>

class Move
{
public:
    sf::Vector2i start_pos;
    sf::Vector2i end_pos;
    Piece piece_moved;
    bool is_capture;
    // Pawn type means it is not a promotion
    // King type means some form of regime change
    // Rook type with king means castle
    Piece::Type promotion_type = Piece::Type::Pawn;

    bool operator==(const Move &other) const;
    bool operator!=(const Move &other) const;
};

sf::Packet &operator<<(sf::Packet &packet, const Move &move);
sf::Packet &operator>>(sf::Packet &packet, Move &move);

#endif