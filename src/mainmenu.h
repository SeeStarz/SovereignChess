#ifndef MAINMENU_H
#define MAINMENU_H

#include "button.h"
#include "loadable.h"
#include "event.h"
#include "boardmanager.h"
#include <SFML/Graphics.hpp>
#include <SFML/Network.hpp>
#include <map>
#include <thread>
#include <array>
#include <string>

class ConnectThread
{
private:
    bool force_stop = false;
    std::thread thread;
    sf::TcpSocket *socket = NULL;
    sf::IpAddress ip = sf::IpAddress::None;
    unsigned int port = 0;
    void runThread();

public:
    bool running = false;
    sf::Socket::Status status = sf::Socket::NotReady;

    void run(sf::TcpSocket *socket, sf::IpAddress ip, unsigned int port);
    void stop();
};

class MainMenu
{
private:
    sf::RenderWindow &window;
    sf::TcpListener listener;
    sf::TcpSocket socket;
    std::map<std::string, OtherButton> buttons;
    std::array<TextFieldButton, 2> text_fields;
    Loadable loadable;
    bool in_menu;
    sf::IpAddress ip;
    unsigned short port;
    ConnectThread thread;
    bool connecting;
    bool hosting;

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
    void tick();
};

#endif