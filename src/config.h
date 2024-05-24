#ifndef CONFIG_H
#define CONFIG_H

#include <array>
#include <vector>
#include <SFML/Graphics.hpp>

class Config
{
private:
    Config(std::string path);
    ~Config();

    static Config *config_;

    static sf::Color strToColor(std::vector<std::string> values);

public:
    Config(Config &other) = delete;
    void operator=(const Config &other) = delete;

    static const Config &getConfig();

    std::array<sf::Color, 12> colors;
    std::string img_dir;
    std::string audio_dir;
    std::string game_dir;
    std::string font_path;
};

#endif
