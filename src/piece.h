#ifndef PIECE_H
#define PIECE_H

#include "texture.h"
#include <SFML/Graphics.hpp>
class Piece
{
public:
    enum Type
    {
        King,
        Queen,
        Rook,
        Bishop,
        Knight,
        Pawn
    };
    Piece(const sf::Vector2i &pos, int faction, int main_owner, int direct_owner, const Type type);
    Piece() = default;
    bool operator==(const Piece &other);
    bool operator!=(const Piece &other);

    sf::Vector2i pos;
    int faction;
    int main_owner;
    int direct_owner;
    Type type;
    bool is_alive = true;
};

#endif