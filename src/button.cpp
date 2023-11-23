#include "button.h"
#include "event.h"
#include <SFML/Graphics.hpp>

TileButton::TileButton(sf::FloatRect rect, sf::Vector2i pos, int layer, bool active)
{
    this->rect = rect;
    this->pos = pos;
    this->layer = layer;
    this->active = active;
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