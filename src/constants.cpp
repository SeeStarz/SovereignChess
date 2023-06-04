#include <SFML/Graphics.hpp>
#include <array>

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

const std::array<sf::Color, 12> colors = {sf::Color(255, 255, 255),
                                          sf::Color(31, 159, 31),
                                          sf::Color(191, 191, 191),
                                          sf::Color(31, 191, 191),
                                          sf::Color(31, 31, 191),
                                          sf::Color(159, 0, 223),
                                          sf::Color(223, 159, 223),
                                          sf::Color(191, 31, 31),
                                          sf::Color(223, 95, 31),
                                          sf::Color(95, 95, 95),
                                          sf::Color(255, 255, 31),
                                          sf::Color(31, 31, 31)};
