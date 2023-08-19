#include "button.h"
#include "event.h"
#include <SFML/Graphics.hpp>

TileButton::TileButton(sf::FloatRect rect, sf::Vector2i pos)
{
    this->rect = rect;
    this->pos = pos;
}

void TileButton::press()
{
    ButtonEventChannel<TileButton>::press(*this);
}

void TileButton::hold()
{
    ButtonEventChannel<TileButton>::hold(*this);
}

void TileButton::release()
{
    ButtonEventChannel<TileButton>::release(*this);
}