#ifndef LOADABLE_H
#define LOADABLE_H

#include <SFML/Graphics.hpp>
#include <SFML/Audio.hpp>
#include <stdexcept>
#include <array>

class Loadable
{
public:
    std::array<sf::Texture, 6> piece_main;
    std::array<sf::Texture, 6> piece_base;
    std::array<sf::Texture, 6> piece_neutral;
    std::array<sf::SoundBuffer, 4> sounds;
    sf::Font font;
    sf::Texture move_marker;

    void load();
};

#endif