#ifndef BUTTON_H
#define BUTTON_H

#include "piece.h"
#include <SFML/Graphics/Rect.hpp>
class Button
{
public:
    sf::FloatRect rect;
    int layer;
    bool hover;
    bool active;
    virtual void press() = 0;
    virtual void hold() = 0;
    virtual void release() = 0;
};

class TileButton : public Button
{
public:
    TileButton() = default;
    TileButton(sf::FloatRect rect, int layer, sf::Vector2i pos, bool active = true);

    sf::Vector2i pos;
    void press();
    void hold();
    void release();
};

class PromotionButton : public Button
{
public:
    PromotionButton() = default;
    PromotionButton(sf::FloatRect rect, int layer, sf::Vector2i pos, Piece::Type promotion_type, bool active = true);

    sf::Vector2i pos; // Refers to the destination pos
    Piece::Type promotion_type;
    void press();
    void hold();
    void release();
};

class OtherButton : public Button
{
public:
    OtherButton() = default;
    OtherButton(sf::FloatRect rect, int layer, std::string identifier, bool active = true);

    std::string identifier;
    void press();
    void hold();
    void release();
};

class TextFieldButton : public Button
{
public:
    TextFieldButton() = default;
    TextFieldButton(sf::FloatRect rect, int layer, bool active = true);

    std::string text;
    void press();
    void hold();
    void release();
};

#endif