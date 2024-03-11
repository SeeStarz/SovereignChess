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
#include <map>
#include <string>

class BoardManager
{
private:
    std::vector<GameState> game_states;
    std::vector<Move> played_moves;
    std::vector<std::vector<Move>> legal_moves;
    std::array<TileButton, 256> tile_buttons;
    std::array<PromotionButton, 5> promotion_buttons;
    std::map<std::string, OtherButton> other_buttons;
    sf::RenderWindow &window;
    Texture texture;
    Piece *selected_piece;
    Piece defection_piece;
    bool swap;
    unsigned int t_listener_id;
    unsigned int p_listener_id;
    unsigned int o_listener_id;

    void drawSquare(int x, int y, sf::Color color);
    void drawBoard();
    void drawPieces();
    void drawExtra();
    void drawPiece(const Piece &piece);
    void drawPiece(const Piece &piece, const sf::FloatRect &rect);
    void drawMoves();

    void onPress(TileButton &button);
    void onHold(TileButton &button);
    void onRelease(TileButton &button);
    void onPress(PromotionButton &button);
    void onHold(PromotionButton &button);
    void onRelease(PromotionButton &button);
    void onPress(OtherButton &button);
    void onHold(OtherButton &button);
    void onRelease(OtherButton &button);
    void registerListener();

    Tile getTile(TileButton &button, int index = -1);
    Tile getTile(sf::Vector2i pos, int index = -1);
    bool isMoveValid(const Move &move, int index = -1);
    bool doMove(const Move &move);
    void refreshButtons();

public:
    BoardManager(sf::RenderWindow &window);
    void registerButtons(std::vector<Button *> &buttons);
    void draw();
};

#endif