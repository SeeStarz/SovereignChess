#ifndef DISPLAYCONFIG_H
#define DISPLAYCONFIG_H

#include <SFML/System/Vector2.hpp>

const sf::Vector2f offset = {10.f, 10.f};
const int height = 788;
const int width = 1162;
const int sprite_size = 16;
const float scale = 3.f;
const float tile_size = sprite_size * scale;
const float line_scale = 1.f;
const int text_size = 32;
const int text_height = 25;
const int text_down = text_size - text_height;
const sf::Vector2f text_offset = {offset.x * 3 + tile_size * 16, offset.y};

#endif