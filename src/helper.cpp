#include "helper.h"
#include <SFML/Graphics.hpp>
#include <tuple>

bool Vector2iLess::operator()(const sf::Vector2i &lhs, const sf::Vector2i &rhs) const
{
    return std::tie(lhs.x, lhs.y) < std::tie(rhs.x, rhs.y);
}
