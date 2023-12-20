#ifndef BOARDMANAGER_H
#define BOARDMANAGER_H

#include "helper.h"
#include "displayconfig.h"
#include "gamestate.h"
#include "texture.h"
#include "piece.h"
#include "move.h"
#include "button.h"
#include <SFML/Graphics.hpp>
#include <vector>

class BoardManager
{
private:
    std::vector<GameState> game_states;
    std::vector<Move> played_moves;
    std::vector<std::vector<Move>> legal_moves;
    sf::RenderWindow &window;
    Texture texture;
    Piece *selected_piece;
    unsigned int listener_id;

    void drawSquare(int x, int y, sf::Color color);
    void drawBoard();
    void drawPieces();
    void drawMoves();

    void onPress(TileButton &button);
    void onHold(TileButton &button);
    void onRelease(TileButton &button);
    void onPress(PromotionButton &button);
    void onHold(PromotionButton &button);
    void onRelease(PromotionButton &button);
    void registerListener();

    Tile getTile(TileButton &button, int index = -1);
    Tile getTile(sf::Vector2i pos, int index = -1);
    bool isMoveValid(const Move &move, int index = -1);

public:
    std::array<TileButton, 256> tile_buttons;
    std::array<PromotionButton, 5> promotion_buttons;
    BoardManager(sf::RenderWindow &window);
    void registerButtons(std::vector<Button *> &buttons);
    void draw();
};

#endif