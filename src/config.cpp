#include "config.h"
#include "helper.h"
#include <map>
#include <string>
#include <cassert>

Config *Config::config_ = NULL;

Config::~Config()
{
    if (config_)
        delete config_;
}

Config::Config(std::string path)
{
    std::map<std::string, std::string> token_to_value;
    token_to_value = readFromFile("..\\config\\config.cfg");
    for (int i = 0; i < 12; i++)
    {
        std::string token = "color" + std::to_string(i);
        auto it = token_to_value.find(token);
        assert(it != token_to_value.end());
        std::vector<std::string> values = extractValues(it->second);
        colors[i] = strToColor(values);
    }
}

const Config &Config::getConfig()
{
    if (!config_)
        config_ = new Config("..\\config\\config.cfg");

    return *config_;
}

sf::Color Config::strToColor(std::vector<std::string> values)
{
    assert(values.size() == 3 || values.size() == 4);
    int transparancy = values.size() == 4 ? std::stoi(values[3]) : 255;
    sf::Color color = sf::Color(std::stoi(values[0]), std::stoi(values[1]), std::stoi(values[2]));
    return color;
}
