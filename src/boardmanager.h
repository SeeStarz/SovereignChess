#ifndef BOARDMANAGER_H
#define BOARDMANAGER_H

#include "helper.h"
#include "displayconfig.h"
#include "gamestate.h"
#include "loadable.h"
#include "piece.h"
#include "move.h"
#include "button.h"
#include <SFML/Graphics.hpp>
#include <SFML/Network.hpp>
#include <SFML/Audio.hpp>
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
    Loadable loadable;
    Piece *selected_piece;
    Piece defection_piece;
    sf::TcpSocket *socket;
    bool player1_is_white;
    bool swap_done;
    unsigned int t_listener_id;
    unsigned int p_listener_id;
    unsigned int o_listener_id;
    sf::Sound sound_player;

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
    void refreshOtherButtons();

    void isGameDone();

public:
    BoardManager(sf::RenderWindow &window);
    // Player 1 is the user playing, socket=NULL indicates offline play
    void startGame(bool player1_is_white = true, sf::TcpSocket *socket = NULL);
    void registerButtons(std::vector<Button *> &buttons);
    void disableButtons();
    void enableButtons();
    void checkNetwork();
    void draw();

    // 0 = running, 1 = checkmate, -1 = checkmated, 2 = stalemate
    // from player 1 standpoint
    int checkmate;
};

#endif