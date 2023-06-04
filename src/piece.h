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
    Piece(const sf::Vector2<int> &pos, int faction, int owner, const Type type, Texture &texture);
    void updateSprite();

    sf::Vector2<int> pos;
    int faction;
    int owner;
    Type type;
    sf::Sprite main_sprite;
    sf::Sprite base_sprite;
    Texture &texture;
};

#endif