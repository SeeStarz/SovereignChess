#include "loadable.h"
#include "config.h"

void Loadable::load()
{
    const Config &config = Config::getConfig();
    
    std::array<std::string, 6> piece_names = {"queen", "knight", "rook", "bishop", "king", "pawn"};
    for (int i = 0; i < 6; i++)
    {
        if (!piece_main[i].loadFromFile(config.img_dir + piece_names[i] + ".png"))
            throw std::runtime_error("");
        if (!piece_base[i].loadFromFile(config.img_dir + piece_names[i] + "_base.png"))
            throw std::runtime_error("");
        if (!piece_neutral[i].loadFromFile(config.img_dir + piece_names[i] + "_neutral.png"))
            throw std::runtime_error("");
    }

    if (!font.loadFromFile(config.font_path))
        throw std::runtime_error("");

    std::array<std::string, 4> audio_names = {"start", "move", "capture", "slide"};
    for (int i = 0; i < 4; i++)
    {
        if (!sounds[i].loadFromFile(config.audio_dir + audio_names[i] + ".mp3"))
            throw std::runtime_error("");
    }

    if (!move_marker.loadFromFile(config.img_dir + "move_marker.png"))
        throw std::runtime_error("");

    if (!colored_tile.loadFromFile(config.img_dir + "colored_tile.png"))
        throw std::runtime_error("");
}
