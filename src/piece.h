#ifndef PIECE_H
#define PIECE_H

#include "loadable.h"
#include <SFML/System/Vector2.hpp>
#include <SFML/Network/Packet.hpp>
#include <array>

class Piece
{
public:
    enum Type
    {
        Queen,
        Knight,
        Rook,
        Bishop,
        King,
        Pawn
    };
    Piece(const sf::Vector2i &pos, int faction, int main_owner, int direct_owner, const Type type);
    Piece() = default;
    bool operator==(const Piece &other) const;
    bool operator!=(const Piece &other) const;

    sf::Vector2i pos;
    int faction;
    int main_owner;
    int direct_owner;
    Type type;
    bool is_alive = true;
};

sf::Packet &operator<<(sf::Packet &packet, const Piece &piece);
sf::Packet &operator>>(sf::Packet &packet, Piece &piece);

#endif