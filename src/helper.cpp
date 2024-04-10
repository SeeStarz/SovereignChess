#include "helper.h"
#include "displayconfig.h"
#include <SFML/Graphics.hpp>
#include <tuple>
#include <cmath>

bool Vector2iLess::operator()(const sf::Vector2i &lhs, const sf::Vector2i &rhs) const
{
    return std::tie(lhs.x, lhs.y) < std::tie(rhs.x, rhs.y);
}

std::vector<sf::Vector2i> getInBetweens(sf::Vector2i pos1, sf::Vector2i pos2)
{
    std::vector<sf::Vector2i> pos_in_between = {};

    sf::Vector2i displacement = pos2 - pos1;
    if (displacement.x == 0)
        for (int i = 1; i < abs(displacement.y); i++)
        {
            int delta_y = (displacement.y * i) / abs(displacement.y);
            pos_in_between.push_back(sf::Vector2i{pos1.x, pos1.y + delta_y});
        }
    else if (displacement.y == 0)
        for (int i = 1; i < abs(displacement.x); i++)
        {
            int delta_x = (displacement.x * i) / abs(displacement.x);
            pos_in_between.push_back(sf::Vector2i{pos1.x + delta_x, pos1.y});
        }
    else if (abs(displacement.x) == abs(displacement.y))
        for (int i = 0; i < abs(displacement.x); i++)
        {
            int delta_x = (displacement.x * i) / abs(displacement.x);
            int delta_y = (displacement.y * i) / abs(displacement.y);
            pos_in_between.push_back(sf::Vector2i{pos1.x + delta_x, pos1.y + delta_y});
        }

    return pos_in_between;
}

float getDistance(sf::Vector2i pos1, sf::Vector2i pos2)
{
    return std::sqrt(std::pow(pos2.x - pos1.x, 2) + std::pow(pos2.y - pos1.y, 2));
}

void alignText(sf::Text &text, const sf::FloatRect &rect)
{
    sf::FloatRect text_rect = text.getLocalBounds();
    text.setPosition(rect.left + (rect.width - text_rect.width) / 2, rect.top + (rect.height - text_size - text_down) / 2);
}