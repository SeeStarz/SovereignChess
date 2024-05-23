#include "loadable.h"
#include "helper.h"

void Loadable::load()
{
    std::array<std::string, 6> piece_names = {"queen", "knight", "rook", "bishop", "king", "pawn"};
    for (int i = 0; i < 6; i++)
    {
        if (!piece_main[i].loadFromFile(img_dir + piece_names[i] + ".png"))
            throw std::runtime_error("");
        if (!piece_base[i].loadFromFile(img_dir + piece_names[i] + "_base.png"))
            throw std::runtime_error("");
        if (!piece_neutral[i].loadFromFile(img_dir + piece_names[i] + "_neutral.png"))
            throw std::runtime_error("");
    }

    if (!font.loadFromFile(font_dir + "Monocraft.ttf"))
        throw std::runtime_error("");

    std::array<std::string, 4> audio_names = {"start", "move", "capture", "slide"};
    for (int i = 0; i < 4; i++)
    {
        if (!sounds[i].loadFromFile(audio_dir + audio_names[i] + ".mp3"))
            throw std::runtime_error("");
    }

    if (!move_marker.loadFromFile(img_dir + "move_marker.png"))
        throw std::runtime_error("");

    if (!colored_tile.loadFromFile(img_dir + "colored_tile.png"))
        throw std::runtime_error("");
}
