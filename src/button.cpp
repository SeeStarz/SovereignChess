#include "button.h"
#include "event.h"
#include "piece.h"
#include <SFML/Graphics.hpp>

TileButton::TileButton(sf::FloatRect rect, int layer, sf::Vector2i pos, bool active)
{
    this->rect = rect;
    this->layer = layer;
    this->pos = pos;
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

PromotionButton::PromotionButton(sf::FloatRect rect, int layer, sf::Vector2i pos, Piece::Type promotion_type, bool active)
{
    this->rect = rect;
    this->layer = layer;
    this->pos = pos;
    this->promotion_type = promotion_type;
    this->active = active;
}

void PromotionButton::press()
{
    ButtonEventChannel<PromotionButton>::press(*this);
}

void PromotionButton::hold()
{
    ButtonEventChannel<PromotionButton>::hold(*this);
}

void PromotionButton::release()
{
    ButtonEventChannel<PromotionButton>::release(*this);
}