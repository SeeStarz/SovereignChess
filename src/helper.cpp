#include "helper.h"
#include "displayconfig.h"
#include <SFML/Graphics.hpp>
#include <tuple>
#include <cmath>
#include <fstream>
#include <sstream>

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

std::map<std::string, std::string> readFromFile(std::string path)
{
    std::map<std::string, std::string> token_to_value;

    std::ifstream file = std::ifstream(path);
    std::string line;
    while (std::getline(file, line))
    {
        std::stringstream ss(line);
        char ch;
        bool stop_reading = false;
        std::string token;
        std::string value;
        bool reading_token = true;
        while (ss >> ch)
        {
            if (ch == '#')
            {
                stop_reading = true;
                break;
            }
            else if (ch == '=')
            {
                if (!reading_token)
                {
                    stop_reading = true;
                    break;
                }
                reading_token = false;
            }
            else if (reading_token)
                token += ch;
            else
                value += ch;
        }
        if (stop_reading)
            continue;

        token_to_value[token] = value;
    }
    file.close();

    return token_to_value;
}

std::vector<std::string> extractValues(std::string value, char separator)
{
    std::vector<std::string> values;
    std::string current_value = "";
    std::stringstream ss(value);
    char ch;
    while (ss >> ch)
    {
        if (ch == separator)
        {
            values.push_back(current_value);
            current_value = "";
        }
        else
        {
            current_value += ch;
        }
    }
    if (current_value != "")
        values.push_back(current_value);

    return values;
}