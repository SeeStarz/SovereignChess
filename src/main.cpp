#include "piece.h"
#include "texture.h"
#include "helper.h"
#include "displayconfig.h"
#include "gamestate.h"
#include "move.h"
#include "boardmanager.h"
#include <SFML/Graphics.hpp>
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

    std::vector<Button *> buttons;
    BoardManager board_manager(window);
    board_manager.registerButtons(buttons);

    bool hold = false;
    Button *clicked_button;

    while (window.isOpen())
    {
        sf::Event event;
        while (window.pollEvent(event))
        {
            if (event.type == sf::Event::Closed)
                window.close();
            if (event.type == sf::Event::MouseButtonPressed && !hold)
            {
                clicked_button = buttonAtPos(buttons, sf::Mouse::getPosition(window));
                if (!clicked_button)
                    continue;
                clicked_button->press();
                hold = true;
            }
            if (event.type == sf::Event::MouseButtonReleased)
            {
                assert(clicked_button);
                clicked_button->release();
                hold = false;
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

        window.clear(sf::Color(255, 255, 255));
        board_manager.draw();
        window.display();
    }
}