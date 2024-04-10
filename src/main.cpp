#include "piece.h"
#include "texture.h"
#include "helper.h"
#include "displayconfig.h"
#include "gamestate.h"
#include "move.h"
#include "boardmanager.h"
#include "mainmenu.h"
#include <SFML/Graphics.hpp>
#include <SFML/Network.hpp>
#include <memory>
#include <iostream>
#include <vector>
#include <array>
#include <map>
#include <cassert>

Button *buttonAtPos(std::vector<Button *> buttons, sf::Vector2i mouse_pos)
{
    Button *clicked_button = NULL;
    for (Button *button : buttons)
        if (button->active && button->rect.contains(sf::Vector2f(mouse_pos)))
            if (!clicked_button || button->layer > clicked_button->layer)
                clicked_button = button;
    return clicked_button;
}

int main()
{
    sf::RenderWindow window(sf::VideoMode(width, height), "Sovereign Chess");
    Texture texture;
    texture.load();

    sf::Image icon;
    if (!icon.loadFromFile("..\\img\\king.png"))
        throw std::runtime_error("");
    window.setIcon(icon.getSize().x, icon.getSize().y, icon.getPixelsPtr());

    std::vector<Button *> buttons;
    MainMenu main_menu = MainMenu(window);
    main_menu.registerButtons(buttons);

    bool hold = false;
    Button *clicked_button = NULL;

    while (window.isOpen())
    {
        sf::Event event;
        while (window.pollEvent(event))
        {
            switch (event.type)
            {
            case sf::Event::Closed:
            {
                window.close();
                break;
            }
            case sf::Event::MouseButtonPressed:
            {
                if (hold)
                    break;

                clicked_button = buttonAtPos(buttons, sf::Mouse::getPosition(window));
                if (!clicked_button)
                    break;
                clicked_button->press();
                hold = true;
            }
            case sf::Event::MouseButtonReleased:
            {
                if (clicked_button)
                    clicked_button->release();
                hold = false;
            }

            case sf::Event::TextEntered:
            {
                if (!clicked_button)
                    break;
                if (!dynamic_cast<TextFieldButton *>(clicked_button))
                    break;
                TextFieldButton *field = dynamic_cast<TextFieldButton *>(clicked_button);
                if (std::isprint(event.text.unicode))
                    field->text += event.text.unicode;
                else if (event.text.unicode == 8)
                    field->text.pop_back();
            }

            case sf::Event::KeyPressed:
            {
                if (!clicked_button)
                    continue;
                if (!dynamic_cast<TextFieldButton *>(clicked_button))
                    continue;
                TextFieldButton *field = dynamic_cast<TextFieldButton *>(clicked_button);
                if (event.key.control && event.key.code == sf::Keyboard::C)
                    sf::Clipboard::setString(field->text);
                if (event.key.control && event.key.code == sf::Keyboard::V)
                    field->text += sf::Clipboard::getString();
                if (event.key.control && event.key.code == sf::Keyboard::Backspace)
                    field->text = "";
            }
            }
        }

        if (hold)
        {
            assert(clicked_button);
            clicked_button->hold();
        }

        sf::Vector2i pos = sf::Mouse::getPosition(window);
        for (Button *button : buttons)
        {
            if (button->rect.contains(sf::Vector2f(pos)))
                button->hover = true;
            else
                button->hover = false;
        }

        window.clear(sf::Color(161, 102, 47));
        main_menu.draw();
        window.display();
    }
}