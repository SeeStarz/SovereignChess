#include "mainmenu.h"
#include "displayconfig.h"
#include "texture.h"
#include "helper.h"
#include "event.h"
#include <iostream>
#include <string>
#include <functional>
#include <map>

MainMenu::MainMenu(sf::RenderWindow &window) : window(window), board_manager(window)
{
    board_manager.disableButtons();
    texture.load();

    in_menu = true;

    sf::FloatRect rect = sf::FloatRect(width / 2 - text_size * 6, offset.y, text_size * 12, text_size * 2);
    buttons["offline"] = OtherButton(rect, 2, "offline");
    rect.top = offset.y + text_size * 6;
    text_fields[0] = TextFieldButton(rect, 2);
    text_fields[0].text = "50000";
    rect.top = offset.y + text_size * 10;
    text_fields[1] = TextFieldButton(rect, 2);

    rect.top = offset.y + text_size * 13;
    rect.width = text_size * 5;
    buttons["host"] = OtherButton(rect, 2, "host");
    rect.left = width / 2 + text_size;
    buttons["connect"] = OtherButton(rect, 2, "connect");

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
    text.setFont(texture.font);
    text.setFillColor(sf::Color::Black);
    text.setCharacterSize(text_size);
    sf::FloatRect rect;

    rect = buttons["offline"].rect;
    shape.setPosition(rect.left, rect.top);
    window.draw(shape);
    text.setString("Offline Play");
    alignText(text, rect);
    window.draw(text);

    rect.top += text_size * 3;
    text.setString("Multi Player");
    alignText(text, rect);
    window.draw(text);

    rect = text_fields[0].rect;
    shape.setPosition(rect.left, rect.top);
    shape.setFillColor(sf::Color::White);
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
        button.active = in_menu;

    for (auto it = this->buttons.begin(); it != this->buttons.end(); it++)
        it->second.active = in_menu;
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
        board_manager.enableButtons();
    }

    else if (button.identifier == "host")
    {
        int port = std::stoi(text_fields[0].text);
        sf::IpAddress address;
        if (text_fields[1].text != "")
            address = sf::IpAddress(text_fields[1].text);
        else
            address = sf::IpAddress::Any;

        if (listener.listen(port, address) != sf::Socket::Done)
        {
            std::cout << "cannot listen to port: " << port << std::endl;
            return;
        }

        if (listener.accept(socket) != sf::Socket::Done)
        {
            std::cout << "cannot connect to socket" << std::endl;
            return;
        }

        in_menu = false;
        updateButtons();
        board_manager.enableButtons();
    }
    else if (button.identifier == "connect")
    {
        int port = std::stoi(text_fields[0].text);
        sf::IpAddress address = sf::IpAddress(text_fields[1].text);

        if (socket.connect(address, port) != sf::Socket::Done)
        {
            std::cout << "cannot connect to socket" << std::endl;
            return;
        }

        in_menu = false;
        updateButtons();
        board_manager.enableButtons();
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