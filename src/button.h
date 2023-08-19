#ifndef BUTTON_H
#define BUTTON_H

#include <SFML/Graphics.hpp>
class Button
{
public:
    sf::FloatRect rect;
    bool hover;
    virtual void press() = 0;
    virtual void hold() = 0;
    virtual void release() = 0;
};

class TileButton : public Button
{
public:
    TileButton() = default;
    TileButton(sf::FloatRect rect, sf::Vector2i pos);

    sf::Vector2i pos;
    void press();
    void hold();
    void release();
};

#endif