#ifndef TEXTURE_H
#define TEXTURE_H

#include <SFML/Graphics.hpp>
#include <stdexcept>
#include <array>

class Texture
{
public:
    std::array<sf::Texture, 6> piece_main;
    std::array<sf::Texture, 6> piece_base;
    std::array<sf::Texture, 6> piece_neutral;

    void load();
};

#endif