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
    Piece(const sf::Vector2<int> &pos, int faction, int owner, const Type type);
    Piece() = default;

    sf::Vector2<int> pos;
    int faction;
    int owner;
    Type type;
};

#endif