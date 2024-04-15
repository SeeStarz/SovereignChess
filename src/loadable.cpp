#include "loadable.h"

void Loadable::load()
{
    std::array<std::string, 6> piece_names = {"king", "queen", "rook", "bishop", "knight", "pawn"};
    for (int i = 0; i < 6; i++)
    {
        if (!piece_main[i].loadFromFile("..\\img\\" + piece_names[i] + ".png"))
            throw std::runtime_error("");
        if (!piece_base[i].loadFromFile("..\\img\\" + piece_names[i] + "_base.png"))
            throw std::runtime_error("");
        if (!piece_neutral[i].loadFromFile("..\\img\\" + piece_names[i] + "_neutral.png"))
            throw std::runtime_error("");
    }

    if (!font.loadFromFile("..\\font\\Monocraft.ttf"))
        throw std::runtime_error("");

    std::array<std::string, 4> audio_names = {"start", "move", "capture", "slide"};
    for (int i = 0; i < 4; i++)
    {
        if (!sounds[i].loadFromFile("..\\audio\\" + audio_names[i] + ".mp3"))
            throw std::runtime_error("");
    }
}