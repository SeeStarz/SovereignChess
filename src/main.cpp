#include "piece.h"
#include "loadable.h"
#include "helper.h"
#include "displayconfig.h"
#include "gamestate.h"
#include "move.h"
#include "boardmanager.h"
#include "mainmenu.h"
#include "config.h"
#include <SFML/Graphics.hpp>
#include <SFML/Network.hpp>
#include <memory>
#include <iostream>
#include <vector>
#include <array>
#include <map>
#include <cassert>

Button *buttonAtPos(std::vector<Button *> buttons, sf::Vector2f click_game_pos)
{
    Button *clicked_button = NULL;
    for (Button *button : buttons)
        if (button->active && button->rect.contains(click_game_pos))
            if (!clicked_button || button->layer > clicked_button->layer)
                clicked_button = button;
    return clicked_button;
}

int main(int argc, char* argv[])
{
    std::string path = argc > 1 ? argv[1] : "";
    const Config& config = (argc > 2) ? Config::getConfig(path, argv[2]) : Config::getConfig(path);
    sf::VideoMode mode;
    if (config.maximize)
        mode = sf::VideoMode::getDesktopMode();
    else
        mode = sf::VideoMode(config.screen_width, config.screen_height);
    sf::RenderWindow window(mode , "Sovereign Chess");
    sf::View view(sf::FloatRect(0.f, 0.f, width, height));
    window.setView(view);

    sf::RectangleShape background(sf::Vector2f(width,height));
    background.setFillColor(sf::Color(161, 102, 47));

    sf::Image icon;
    if (!icon.loadFromFile(config.img_dir + "king.png"))
        throw std::runtime_error("");

    window.setIcon(icon.getSize().x, icon.getSize().y, icon.getPixelsPtr());
    window.setFramerateLimit(60);

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

                clicked_button = buttonAtPos(buttons, window.mapPixelToCoords(sf::Mouse::getPosition(window)));
                if (!clicked_button)
                    break;
                clicked_button->press();
                hold = true;
                break;
            }
            case sf::Event::MouseButtonReleased:
            {
                if (clicked_button)
                    clicked_button->release();
                hold = false;
                break;
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
                else if (event.text.unicode == 8 && field->text.size() != 0)
                    field->text.pop_back();
                break;
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
                break;
            }
            case sf::Event::Resized:
            {
                if (!config.force_aspect_ratio)
                    break;

                float ratio = float(width)/height;
                if (float(event.size.width) / event.size.height >= ratio) 
                {
                    float resized_width = event.size.height * ratio;
                    float view_scale = resized_width / event.size.width;
                    float view_offset = (event.size.width - resized_width) / (2*event.size.width);
                    view.setViewport(sf::FloatRect(view_offset, 0.f, view_scale, 1.f));
                }
                else
                {
                    float resized_height = event.size.width / ratio;
                    float view_scale = resized_height / event.size.height;
                    float view_offset = (event.size.height - resized_height) / (2*event.size.height);
                    view.setViewport(sf::FloatRect(0.f, view_offset, 1.f, view_scale));
                } 
                window.setView(view);
                break;
            }
            default:
                break;
            }
        }

        if (hold)
        {
            assert(clicked_button);
            clicked_button->hold();
        }

        sf::Vector2f pos = window.mapPixelToCoords(sf::Mouse::getPosition(window));
        for (Button *button : buttons)
        {
            if (button->rect.contains(pos))
                button->hover = true;
            else
                button->hover = false;
        }

        window.clear();
        window.draw(background);
        main_menu.draw();
        main_menu.tick();
        window.display();
    }
}
