#ifndef CONFIG_H
#define CONFIG_H

#include <array>
#include <vector>
#include <SFML/Graphics.hpp>

class Config
{
private:
    Config(std::string path, std::string fallback);
    ~Config();

    static Config *config_;

    static sf::Color strToColor(std::vector<std::string> values);

public:
    Config(Config &other) = delete;
    void operator=(const Config &other) = delete;

    static const Config &getConfig(std::string path = "" ,std::string fallback = "../config/default.cfg");

    std::array<sf::Color, 12> colors;
    std::string img_dir;
    std::string audio_dir;
    std::string game_dir;
    std::string font_path;

    bool maximize;
    bool force_aspect_ratio;
    unsigned int screen_width;
    unsigned int screen_height;
};

#endif
