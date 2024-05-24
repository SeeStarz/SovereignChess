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
    // TODO CHECK IF CONFIG EXISTS
    token_to_value = readFromFile("../config/config.cfg");
    for (int i = 0; i < 12; i++)
    {
        std::string token = "color" + std::to_string(i);
        std::string color = token_to_value.at(token);
        std::vector<std::string> values = extractValues(color);
        colors[i] = strToColor(values);
    }
    {
        img_dir = token_to_value.at("img_dir");
        audio_dir = token_to_value.at("audio_dir");
        game_dir = token_to_value.at("game_dir");
        font_path = token_to_value.at("font_path");
    }
}

const Config &Config::getConfig()
{
    if (!config_)
        config_ = new Config("../config/config.cfg");

    return *config_;
}

sf::Color Config::strToColor(std::vector<std::string> values)
{
    assert(values.size() == 3 || values.size() == 4);
    int transparancy = values.size() == 4 ? std::stoi(values[3]) : 255;
    sf::Color color = sf::Color(std::stoi(values[0]), std::stoi(values[1]), std::stoi(values[2]), transparancy);
    return color;
}
