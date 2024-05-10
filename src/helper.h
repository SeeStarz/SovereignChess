#ifndef HELPER_H
#define HELPER_H

#include <SFML/Graphics.hpp>
#include <array>
#include <tuple>
#include <vector>
#include <map>

const std::array<sf::Vector2i, 8> castle_rooks_pos = {
    sf::Vector2i(2, 15),
    sf::Vector2i(4, 15),
    sf::Vector2i(11, 15),
    sf::Vector2i(13, 15),
    sf::Vector2i(2, 0),
    sf::Vector2i(4, 0),
    sf::Vector2i(11, 0),
    sf::Vector2i(13, 0)};

class Vector2iLess
{
public:
    bool operator()(const sf::Vector2i &lhs, const sf::Vector2i &rhs) const;
};

// Orthogonal only, ordered from nearest pos1 to pos2
std::vector<sf::Vector2i> getInBetweens(sf::Vector2i pos1, sf::Vector2i pos2);

float getDistance(sf::Vector2i pos1, sf::Vector2i pos2);

const std::map<sf::Vector2i, int, Vector2iLess> color_map = {
    {{7, 7}, 0},
    {{8, 8}, 0},
    {{5, 10}, 1},
    {{10, 5}, 1},
    {{9, 6}, 2},
    {{6, 9}, 2},
    {{10, 7}, 3},
    {{5, 8}, 3},
    {{11, 4}, 4},
    {{4, 11}, 4},
    {{8, 5}, 5},
    {{7, 10}, 5},
    {{7, 5}, 6},
    {{8, 10}, 6},
    {{4, 4}, 7},
    {{11, 11}, 7},
    {{5, 7}, 8},
    {{10, 8}, 8},
    {{6, 6}, 9},
    {{9, 9}, 9},
    {{5, 5}, 10},
    {{10, 10}, 10},
    {{8, 7}, 11},
    {{7, 8}, 11}};

void alignText(sf::Text &text, const sf::FloatRect &rect);

std::map<std::string, std::string> readFromFile(std::string path);
std::vector<std::string> extractValues(std::string value, char separator = ',');

#endif