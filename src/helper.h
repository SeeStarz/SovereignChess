#ifndef HELPER_H
#define HELPER_H

#include <SFML/Graphics.hpp>
#include <array>
#include <tuple>
#include <vector>

enum Color
{
    White,
    Green,
    LightGray,
    Cyan,
    Blue,
    Purple,
    Pink,
    Red,
    Orange,
    DarkGray,
    Yellow,
    Black
};

const std::array<sf::Color, 12> colors =
    {sf::Color(255, 255, 255),
     sf::Color(31, 159, 31),
     sf::Color(223, 223, 223),
     sf::Color(31, 191, 191),
     sf::Color(31, 31, 191),
     sf::Color(159, 0, 223),
     sf::Color(223, 159, 223),
     sf::Color(191, 31, 31),
     sf::Color(223, 95, 31),
     sf::Color(127, 127, 127),
     sf::Color(255, 255, 31),
     sf::Color(63, 63, 63)};

class Vector2iLess
{
public:
    bool operator()(const sf::Vector2i &lhs, const sf::Vector2i &rhs) const;
};

// Orthogonal only
std::vector<sf::Vector2i> getInBetweens(sf::Vector2i pos1, sf::Vector2i pos2);

const std::map<sf::Vector2i, Color, Vector2iLess> color_map = {
    {{7, 7}, Color::White},
    {{8, 8}, Color::White},
    {{5, 10}, Color::Green},
    {{10, 5}, Color::Green},
    {{9, 6}, Color::LightGray},
    {{6, 9}, Color::LightGray},
    {{10, 7}, Color::Cyan},
    {{5, 8}, Color::Cyan},
    {{11, 4}, Color::Blue},
    {{4, 11}, Color::Blue},
    {{8, 5}, Color::Purple},
    {{7, 10}, Color::Purple},
    {{7, 5}, Color::Pink},
    {{8, 10}, Color::Pink},
    {{4, 4}, Color::Red},
    {{11, 11}, Color::Red},
    {{5, 7}, Color::Orange},
    {{10, 8}, Color::Orange},
    {{6, 6}, Color::DarkGray},
    {{9, 9}, Color::DarkGray},
    {{5, 5}, Color::Yellow},
    {{10, 10}, Color::Yellow},
    {{8, 7}, Color::Black},
    {{7, 8}, Color::Black}};

#endif