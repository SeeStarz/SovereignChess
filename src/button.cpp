#include "button.h"
#include "event.h"
#include "piece.h"
#include <SFML/Graphics/Rect.hpp>

TileButton::TileButton(sf::FloatRect rect, int layer, sf::Vector2i pos, bool active)
{
    this->rect = rect;
    this->layer = layer;
    this->pos = pos;
    this->active = active;
    hover = false;
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
    hover = false;
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

OtherButton::OtherButton(sf::FloatRect rect, int layer, std::string identifier, bool active)
{
    this->rect = rect;
    this->layer = layer;
    this->identifier = identifier;
    this->active = active;
    hover = false;
}

void OtherButton::press()
{
    ButtonEventChannel<OtherButton>::press(*this);
}

void OtherButton::hold()
{
    ButtonEventChannel<OtherButton>::hold(*this);
}

void OtherButton::release()
{
    ButtonEventChannel<OtherButton>::release(*this);
}

TextFieldButton::TextFieldButton(sf::FloatRect rect, int layer, bool active)
{
    this->rect = rect;
    this->layer = layer;
    this->active = active;
    hover = false;
}

void TextFieldButton::press()
{
    ButtonEventChannel<TextFieldButton>::press(*this);
}

void TextFieldButton::hold()
{
    ButtonEventChannel<TextFieldButton>::hold(*this);
}

void TextFieldButton::release()
{
    ButtonEventChannel<TextFieldButton>::release(*this);
}