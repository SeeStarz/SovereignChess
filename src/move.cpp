#include "move.h"
#include <SFML/System/Vector2.hpp>
#include <SFML/Network/Packet.hpp>
#include <array>

bool Move::operator==(const Move &other) const
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

bool Move::operator!=(const Move &other) const
{
    return !(*this == other);
}

sf::Packet &operator<<(sf::Packet &packet, const Move &move)
{
    return packet << (sf::Uint8)move.start_pos.x << (sf::Uint8)move.start_pos.y << (sf::Uint8)move.end_pos.x << (sf::Uint8)move.end_pos.y << move.piece_moved << (sf::Uint8)move.is_capture << (sf::Uint8)move.promotion_type;
}

sf::Packet &operator>>(sf::Packet &packet, Move &move)
{
    std::array<sf::Uint8, 6> array;
    for (int i = 0; i < 6; i++)
    {
        if (i == 4)
            packet >> move.piece_moved;
        packet >> array[i];
    }
    move.start_pos = sf::Vector2i(array[0], array[1]);
    move.end_pos = sf::Vector2i(array[2], array[3]);
    move.is_capture = array[4];
    move.promotion_type = (Piece::Type)array[5];
    return packet;
}