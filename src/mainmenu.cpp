#include "mainmenu.h"
#include "displayconfig.h"
#include "loadable.h"
#include "helper.h"
#include "event.h"
#include <iostream>
#include <string>
#include <functional>
#include <thread>
#include <map>
#include <cassert>

MainMenu::MainMenu(sf::RenderWindow &window) : window(window), board_manager(window)
{
    loadable.load();

    // Socket can only accept in blocking mode
    socket.setBlocking(true);
    listener.setBlocking(true);
    in_menu = true;
    connecting = false;
    hosting = false;
    ip = sf::IpAddress::None;
    port = 0;

    sf::FloatRect rect = sf::FloatRect(width / 2 - text_size * 6, offset.y + text_size, text_size * 12, text_size * 2);
    buttons["offline"] = OtherButton(rect, 2, "offline");
    rect.top = offset.y + text_size * 7;
    text_fields[0] = TextFieldButton(rect, 2);
    text_fields[0].text = "50000";
    rect.top = offset.y + text_size * 11;
    text_fields[1] = TextFieldButton(rect, 2);

    rect.top = offset.y + text_size * 14;
    rect.width = text_size * 5;
    buttons["host"] = OtherButton(rect, 2, "host");
    rect.left = width / 2 + text_size;
    buttons["connect"] = OtherButton(rect, 2, "connect");

    rect.left = width / 2 - text_size * 4;
    rect.top = height / 2;
    rect.width = text_size * 8;
    buttons["cancel"] = OtherButton(rect, 3, "cancel");

    updateButtons();
    registerListener();
}

void MainMenu::draw()
{
    if (in_menu)
        drawMenu();
    else
        board_manager.draw();
}

void MainMenu::drawMenu()
{
    sf::RectangleShape shape = sf::RectangleShape(sf::Vector2f(text_size * 12, text_size * 2));
    shape.setFillColor(sf::Color(255, 255, 255, 127));
    sf::Text text;
    text.setFont(loadable.font);
    text.setFillColor(sf::Color::Black);
    text.setCharacterSize(text_size);
    sf::FloatRect rect;

    rect = buttons["offline"].rect;
    shape.setPosition(rect.left, rect.top);
    window.draw(shape);
    text.setString("Offline Play");
    alignText(text, rect);
    window.draw(text);

    rect.top = buttons["offline"].rect.top + text_size * 3;
    rect.left = width / 2 - text_size * 7;
    rect.width = text_size * 14;
    rect.height = text_size * 13;
    shape.setSize(sf::Vector2f(rect.width, rect.height));
    shape.setPosition(rect.left, rect.top);
    shape.setFillColor(sf::Color(127, 127, 127));
    window.draw(shape);

    rect = buttons["offline"].rect;
    rect.top += text_size * 3;
    text.setString("Multi Player");
    alignText(text, rect);
    window.draw(text);

    rect = text_fields[0].rect;
    shape.setPosition(rect.left, rect.top);
    shape.setFillColor(sf::Color::White);
    shape.setSize(sf::Vector2f(rect.width, rect.height));
    window.draw(shape);
    text.setString(text_fields[0].text);
    alignText(text, rect);
    window.draw(text);

    text.setString("Port:");
    rect.top = rect.top - text_size;
    rect.height = text_size;
    alignText(text, rect);
    window.draw(text);

    rect = text_fields[1].rect;
    shape.setPosition(rect.left, rect.top);
    window.draw(shape);
    text.setString(text_fields[1].text);
    alignText(text, rect);
    window.draw(text);

    text.setString("Host Address:");
    rect.top = rect.top - text_size;
    rect.height = text_size;
    alignText(text, rect);
    window.draw(text);

    shape.setSize(sf::Vector2f(text_size * 5, text_size * 2));
    rect = buttons["host"].rect;
    shape.setPosition(rect.left, rect.top);
    shape.setFillColor(sf::Color(255, 255, 255, 127));
    window.draw(shape);
    text.setString("Host");
    alignText(text, rect);
    window.draw(text);

    rect = buttons["connect"].rect;
    shape.setPosition(rect.left, rect.top);
    window.draw(shape);
    text.setString("Connect");
    alignText(text, rect);
    window.draw(text);

    if (connecting)
    {
        shape.setFillColor(sf::Color(86, 50, 50));
        shape.setPosition(width / 2 - text_size * 10, height / 2 - text_size * 3);
        shape.setSize(sf::Vector2f(text_size * 20, text_size * 6));
        window.draw(shape);

        shape.setFillColor(sf::Color(255, 255, 255, 127));
        rect = buttons["cancel"].rect;
        shape.setPosition(rect.left, rect.top);
        shape.setSize(sf::Vector2f(rect.width, rect.height));
        window.draw(shape);
        text.setString("Cancel");
        alignText(text, rect);
        window.draw(text);

        rect.top -= text_size * 2;
        text.setString("Waiting for connection...");
        alignText(text, rect);
        window.draw(text);
    }
}

void MainMenu::tick()
{
    if (!in_menu)
    {
        board_manager.checkNetwork();
        return;
    }

    if (!thread.running && thread.status == sf::Socket::Done)
    {
        in_menu = false;
        connecting = false;
        thread.stop();
        thread.status = sf::Socket::Disconnected;
        updateButtons();
        board_manager.startGame(false, &socket);
    }

    if (!connecting)
        return;

    sf::Socket::Status status;
    if (hosting)
    {
        status = listener.accept(socket);
        if (status == sf::Socket::Done)
        {
            in_menu = false;
            connecting = false;
            listener.close();
            updateButtons();
            board_manager.startGame(true, &socket);
        }
        else
            assert(status == sf::Socket::NotReady);
    }
    else if (!thread.running)
    {
        thread.run(&socket, ip, port);
    }
}

void MainMenu::registerButtons(std::vector<Button *> &buttons)
{
    board_manager.registerButtons(buttons);

    for (TextFieldButton &button : text_fields)
        buttons.push_back(&button);

    for (auto it = this->buttons.begin(); it != this->buttons.end(); it++)
        buttons.push_back(&it->second);
}

void MainMenu::updateButtons()
{
    for (TextFieldButton &button : text_fields)
        button.active = in_menu && !connecting;

    for (auto it = this->buttons.begin(); it != this->buttons.end(); it++)
        it->second.active = in_menu && !connecting;

    buttons["cancel"].active = in_menu && connecting;
}

void MainMenu::onPress(TextFieldButton &button)
{
}
void MainMenu::onHold(TextFieldButton &button)
{
}
void MainMenu::onRelease(TextFieldButton &button)
{
}

void MainMenu::onPress(OtherButton &button)
{
    if (button.identifier == "offline")
    {
        in_menu = false;
        updateButtons();
        board_manager.startGame(true, NULL);
    }
    else if (button.identifier == "host")
    {
        port = std::stoi(text_fields[0].text);
        if (text_fields[1].text != "")
            ip = sf::IpAddress(text_fields[1].text);
        else
            ip = sf::IpAddress::Any;

        if (listener.listen(port, ip) != sf::Socket::Done)
            std::cout << "Failed to listen to port: " << port << std::endl;
        else
        {
            listener.setBlocking(false);

            connecting = true;
            hosting = true;
            updateButtons();
        }
    }
    else if (button.identifier == "connect")
    {
        port = std::stoi(text_fields[0].text);
        ip = sf::IpAddress(text_fields[1].text);

        connecting = true;
        hosting = false;
        updateButtons();
    }
    else if (button.identifier == "cancel")
    {
        connecting = false;
        listener.close();
        thread.stop();
        updateButtons();
    }

    else if (button.identifier == "exityes")
    {
        in_menu = true;
        connecting = false;
        hosting = false;
        updateButtons();
        socket.disconnect();
        socket.setBlocking(true);
    }
}
void MainMenu::onHold(OtherButton &button)
{
}
void MainMenu::onRelease(OtherButton &button)
{
}

void MainMenu::registerListener()
{
    using ffunc = std::function<void(TextFieldButton & button)>;
    void (MainMenu::*tfop)(TextFieldButton &button) = &onPress;
    void (MainMenu::*tfoh)(TextFieldButton &button) = &onHold;
    void (MainMenu::*tfor)(TextFieldButton &button) = &onRelease;

    ffunc tfpress = std::bind(tfop, this, std::placeholders::_1);
    ffunc tfhold = std::bind(tfoh, this, std::placeholders::_1);
    ffunc tfrelease = std::bind(tfor, this, std::placeholders::_1);
    o_listener_id = ButtonEventChannel<TextFieldButton>::registerListener(tfpress, tfhold, tfrelease);

    using ofunc = std::function<void(OtherButton & button)>;
    void (MainMenu::*oop)(OtherButton &button) = &onPress;
    void (MainMenu::*ooh)(OtherButton &button) = &onHold;
    void (MainMenu::*oor)(OtherButton &button) = &onRelease;

    ofunc opress = std::bind(oop, this, std::placeholders::_1);
    ofunc ohold = std::bind(ooh, this, std::placeholders::_1);
    ofunc orelease = std::bind(oor, this, std::placeholders::_1);
    o_listener_id = ButtonEventChannel<OtherButton>::registerListener(opress, ohold, orelease);
}

void ConnectThread::runThread()
{
    force_stop = false;
    while (!force_stop)
    {
        running = true;
        assert(socket);
        status = socket->connect(ip, port, sf::seconds(5));
        if (status == sf::Socket::Done)
        {
            running = false;
            return;
        }
        std::cout << "Can't connect, status: " << status << std::endl;
    }
    running = false;
}

void ConnectThread::stop()
{
    if (thread.joinable())
        thread.detach();
    force_stop = true;
}

void ConnectThread::run(sf::TcpSocket *socket, sf::IpAddress ip, unsigned int port)
{
    this->socket = socket;
    this->ip = ip;
    this->port = port;
    if (thread.joinable())
    {
        force_stop = true;
        thread.join();
    }
    running = true;
    thread = std::thread(ConnectThread::runThread, this);
}