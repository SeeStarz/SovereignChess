#ifndef MAINMENU_H
#define MAINMENU_H

#include "button.h"
#include "texture.h"
#include "event.h"
#include "boardmanager.h"
#include <SFML/Graphics.hpp>
#include <SFML/Network.hpp>
#include <map>
#include <array>
#include <string>

class MainMenu
{
private:
    sf::RenderWindow &window;
    sf::TcpListener listener;
    sf::TcpSocket socket;
    std::map<std::string, OtherButton> buttons;
    std::array<TextFieldButton, 2> text_fields;
    Texture texture;
    bool in_menu;

    unsigned int o_listener_id;
    unsigned int f_listener_id;

    void onPress(TextFieldButton &button);
    void onHold(TextFieldButton &button);
    void onRelease(TextFieldButton &button);
    void onPress(OtherButton &button);
    void onHold(OtherButton &button);
    void onRelease(OtherButton &button);
    void registerListener();

    void updateButtons();

    void drawMenu();

public:
    BoardManager board_manager;

    MainMenu(sf::RenderWindow &window);
    void registerButtons(std::vector<Button *> &buttons);
    void draw();
};

#endif