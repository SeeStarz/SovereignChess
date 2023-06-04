#include <SFML/Graphics.hpp>
#include <array>
#include <stdexcept>

std::array<sf::Texture, 6> piece_main;
std::array<sf::Texture, 6> piece_base;
std::array<sf::Texture, 6> piece_neutral;

void load()
{
    std::array<std::string, 6> names = {"king", "queen", "rook", "bishop", "knight", "pawn"};
    for (int i = 0; i < 6; i++)
    {
        if (!piece_main[i].loadFromFile("..\\img\\" + names[i] + ".png"))
            throw std::runtime_error("");
        if (!piece_base[i].loadFromFile("..\\img\\" + names[i] + "_base.png"))
            throw std::runtime_error("");
        if (!piece_neutral[i].loadFromFile("..\\img\\" + names[i] + "_neutral.png"))
            throw std::runtime_error("");
    }
}