#include "boardmanager.h"
#include "helper.h"
#include "displayconfig.h"
#include "piece.h"
#include "move.h"
#include "event.h"
#include "config.h"
#include "button.h"
#include <SFML/Graphics.hpp>
#include <SFML/Network.hpp>
#include <SFML/Audio.hpp>
#include <string>
#include <utility>
#include <cassert>
#include <iostream>
#include <functional>
#include <fstream>
#include <sstream>

BoardManager::BoardManager(sf::RenderWindow &window) : window(window), config(Config::getConfig())
{
    loadable = Loadable();
    loadable.load();

    selected_piece = NULL;
    socket = NULL;
    player1_is_white = true;
    swap_done = false;
    t_listener_id = -1;
    p_listener_id = -1;
    o_listener_id = -1;
    registerListener();

    for (int x = 0; x < 16; x++)
    {
        for (int y = 0; y < 16; y++)
        {
            int i = 16 * x + y;
            tile_buttons[i] = TileButton{sf::FloatRect(offset + sf::Vector2f(tile_size * x, tile_size * y), sf::Vector2f(tile_size, tile_size)), 0, sf::Vector2i(x, y), false};
        }
    }

    std::array<Piece::Type, 5> piece_types = {Piece::Type::Queen, Piece::Type::Knight, Piece::Type::Rook, Piece::Type::Bishop, Piece::Type::King};
    for (int i = 0; i < 5; i++)
    {
        promotion_buttons[i] = PromotionButton{sf::FloatRect(0, 0, tile_size, tile_size), 1, {0, 0}, piece_types[i], false};
    }

    for (int i = 0; i < 12; i++)
    {
        std::string identifier = "king" + std::to_string(i);
        sf::FloatRect rect;
        if (i < 6)
            rect = sf::FloatRect(tile_size * i + text_offset.x, height - offset.y - tile_size * 2, tile_size, tile_size);
        else
            rect = sf::FloatRect(tile_size * (i - 6) + text_offset.x, height - offset.y - tile_size, tile_size, tile_size);
        other_buttons[identifier] = OtherButton(rect, 0, identifier, false);
    }

    {
        sf::FloatRect rect = sf::FloatRect(text_offset.x, text_offset.y + text_size * 5.5f, 5 * text_size, 2 * text_size);
        other_buttons["swap1"] = OtherButton(rect, 0, "swap1", false);
        rect.left = rect.left + rect.width + text_size;
        other_buttons["swap0"] = OtherButton(rect, 0, "swap0", false);
    }

    {
        sf::FloatRect rect = sf::FloatRect(text_offset.x, height - offset.y - tile_size * 2 - text_size * 3, text_size * 11, text_size * 2);
        other_buttons["exitprompt"] = OtherButton(rect, 0, "exitprompt", false);

        rect = sf::FloatRect(width / 2 - text_size * 6, height / 2, text_size * 5.5f, text_size * 2);
        other_buttons["exityes"] = OtherButton(rect, 1, "exityes", false);
        rect.left = rect.left + rect.width + text_size;
        other_buttons["exitno"] = OtherButton(rect, 1, "exitno", false);

        rect = sf::FloatRect(text_offset.x, height - offset.y - tile_size * 2 - text_size * 6, text_size * 11, text_size * 2);
        other_buttons["savegame"] = OtherButton(rect, 0, "savegame", false);
    }
}

void BoardManager::startGame(bool player1_is_white, sf::TcpSocket *socket)
{
    game_states.clear();
    game_states.push_back(GameState());
    played_moves.clear();
    legal_moves.clear();
    legal_moves.push_back(game_states[0].getMoves());
    selected_piece = NULL;
    this->socket = socket;
    if (socket)
        socket->setBlocking(false);
    this->player1_is_white = player1_is_white;
    swap_done = false;
    swapped = false;

    enableButtons();

    point = 2;
    running = true;

    sound_player.setBuffer(loadable.sounds[0]);
    sound_player.play();
}

void BoardManager::debugGame(std::string file_path)
{
    startGame();

    std::ifstream file = std::ifstream(file_path);
    std::string line;
    while (std::getline(file, line))
    {
        std::stringstream ss(line);
        char ch;
        bool comment = false;
        std::string token;
        std::string value;
        bool reading_token = true;
        while (ss >> ch)
        {
            if (ch == '#')
            {
                comment = true;
                break;
            }
            else if (ch == '=')
            {
                assert(reading_token);
                reading_token = false;
            }
            else if (reading_token)
                token += ch;
            else
                value += ch;
        }
        if (comment)
            continue;

        if (token == "move")
        {
            Move move = strToMove(value);
            assert(doMove(move));
        }
    }
    file.close();

    // Swap button probably turns off other buttons
    enableButtons();
}

void BoardManager::drawSquare(int x, int y, sf::Color color)
{
    sf::RectangleShape square = sf::RectangleShape(sf::Vector2f(tile_size, tile_size));
    square.setFillColor(color);
    square.setPosition(sf::Vector2f(tile_size * x, tile_size * y) + offset);
    window.draw(square);
}

void BoardManager::drawSprite(int x, int y, const sf::Texture &texture, sf::Color color)
{
    sf::Sprite sprite = sf::Sprite(texture);
    sprite.setScale(scale, scale);
    sprite.setPosition(offset + sf::Vector2f(tile_size * x, tile_size * y));
    sprite.setColor(color);
    window.draw(sprite);
}

void BoardManager::drawBoard()
{
    for (int i = 0; i < 16; i++)
    {
        for (int j = 0; j < 16; j++)
        {
            if ((i + j) % 2 == 0)
                drawSquare(i, j, sf::Color(255, 248, 220));
            else
                drawSquare(i, j, sf::Color(222, 184, 135));
        }
    }

    for (auto it = color_map.begin(); it != color_map.end(); it++)
        // drawSquare(it->first.x, it->first.y, colors[it->second]);
        drawSquare(it->first.x, it->first.y, config.colors[it->second]);

    sf::RectangleShape horizontal_line(sf::Vector2f(tile_size * 16, line_scale * 2));
    sf::RectangleShape vertical_line(sf::Vector2f(line_scale * 2, tile_size * 16));
    horizontal_line.setFillColor(sf::Color::Black);
    vertical_line.setFillColor(sf::Color::Black);
    horizontal_line.setOrigin(line_scale, line_scale);
    vertical_line.setOrigin(line_scale, line_scale);
    for (int i = 0; i <= 16; i++)
    {
        horizontal_line.setPosition(sf::Vector2f(0, tile_size * i) + offset);
        vertical_line.setPosition(sf::Vector2f(tile_size * i, 0) + offset);
        window.draw(horizontal_line);
        window.draw(vertical_line);
    }

    sf::RectangleShape board_border(sf::Vector2f(tile_size * 16, tile_size * 16));
    board_border.setPosition(offset);
    board_border.setFillColor(sf::Color::Transparent);
    board_border.setOutlineColor(sf::Color::Black);
    board_border.setOutlineThickness(line_scale * 5);
    window.draw(board_border);

    sf::RectangleShape horizontal_pawn(sf::Vector2f(tile_size * 6, line_scale * 4));
    horizontal_pawn.setOrigin(0, line_scale * 2);
    horizontal_pawn.setFillColor(sf::Color(66, 47, 43));
    horizontal_pawn.setPosition(sf::Vector2f(0, tile_size * 8) + offset);
    window.draw(horizontal_pawn);
    horizontal_pawn.setPosition(sf::Vector2f(tile_size * 10, tile_size * 8) + offset);
    window.draw(horizontal_pawn);

    sf::RectangleShape vertical_pawn(sf::Vector2f(line_scale * 4, tile_size * 6));
    vertical_pawn.setOrigin(line_scale * 2, 0);
    vertical_pawn.setFillColor(sf::Color(66, 47, 43));
    vertical_pawn.setPosition(sf::Vector2f(tile_size * 8, 0) + offset);
    window.draw(vertical_pawn);
    vertical_pawn.setPosition(sf::Vector2f(tile_size * 8, tile_size * 10) + offset);
    window.draw(vertical_pawn);

    sf::RectangleShape promotion_border(sf::Vector2f(tile_size * 4, tile_size * 4));
    promotion_border.setPosition(sf::Vector2f(tile_size * 6, tile_size * 6) + offset);
    promotion_border.setFillColor(sf::Color::Transparent);
    promotion_border.setOutlineColor(sf::Color::Black);
    promotion_border.setOutlineThickness(line_scale * 3);
    window.draw(promotion_border);
}

void BoardManager::drawPieces()
{
    const std::vector<Piece> &pieces = game_states.back().pieces;
    for (int i = 0; i < static_cast<int>(pieces.size()); i++)
    {
        const Piece &piece = pieces[i];
        if (!piece.is_alive)
            continue;

        drawPiece(piece);
    }
}

void BoardManager::drawPiece(const Piece &piece)
{
    sf::Sprite main_sprite(loadable.piece_main[piece.type]);
    main_sprite.setScale(scale, scale);
    // main_sprite.setColor(colors[piece.faction]);
    main_sprite.setColor(config.colors[piece.faction]);
    main_sprite.setPosition(sf::Vector2f(piece.pos.x * tile_size, piece.pos.y * tile_size) + offset);
    window.draw(main_sprite);

    if (piece.main_owner != -1)
    {
        sf::Sprite base_sprite(loadable.piece_base[piece.type]);
        base_sprite.setScale(scale, scale);
        // base_sprite.setColor(colors[piece.main_owner]);
        base_sprite.setColor(config.colors[piece.main_owner]);
        base_sprite.setPosition(sf::Vector2f(piece.pos.x * tile_size, piece.pos.y * tile_size) + offset);
        window.draw(base_sprite);
    }
    else
    {
        sf::Sprite neutral_sprite(loadable.piece_neutral[piece.type]);
        neutral_sprite.setScale(scale, scale);
        neutral_sprite.setPosition(sf::Vector2f(piece.pos.x * tile_size, piece.pos.y * tile_size) + offset);
        window.draw(neutral_sprite);
    }
}

void BoardManager::drawPiece(const Piece &piece, const sf::FloatRect &rect)
{
    sf::Sprite main_sprite(loadable.piece_main[piece.type]);
    main_sprite.setScale(rect.width / sprite_size, rect.height / sprite_size);
    // main_sprite.setColor(colors[piece.faction]);
    main_sprite.setColor(config.colors[piece.faction]);
    main_sprite.setPosition(rect.left, rect.top);
    window.draw(main_sprite);

    if (piece.main_owner != -1)
    {
        sf::Sprite base_sprite(loadable.piece_base[piece.type]);
        base_sprite.setScale(rect.width / sprite_size, rect.height / sprite_size);
        // base_sprite.setColor(colors[piece.main_owner]);
        base_sprite.setColor(config.colors[piece.main_owner]);
        base_sprite.setPosition(rect.left, rect.top);
        window.draw(base_sprite);
    }
    else
    {
        sf::Sprite neutral_sprite(loadable.piece_neutral[piece.type]);
        neutral_sprite.setScale(rect.width / sprite_size, rect.height / sprite_size);
        neutral_sprite.setPosition(rect.left, rect.top);
        window.draw(neutral_sprite);
    }
}

void BoardManager::drawMoves()
{
    assert(legal_moves.size() == game_states.size());
    std::vector<Move> moves = legal_moves.back();

    if (played_moves.size() > 0)
    {
        const Move &move = played_moves.back();
        drawSprite(move.start_pos.x, move.start_pos.y, loadable.move_marker, sf::Color(127, 0, 127));
        drawSprite(move.end_pos.x, move.end_pos.y, loadable.move_marker, sf::Color(127, 0, 127));
    }

    if (!selected_piece)
        return;

    bool in_place = false;
    for (int i = 0; i < static_cast<int>(moves.size()); i++)
    {
        const Move &move = moves[i];
        if (move.start_pos != selected_piece->pos)
            continue;
        if (move.piece_moved.faction != selected_piece->faction)
            continue;
        if (move.piece_moved.type == Piece::Type::Pawn && move.promotion_type != Piece::Type::Pawn && move.promotion_type != Piece::Type::Queen)
            continue;
        if (!in_place && move.start_pos == move.end_pos)
            in_place = true;

        if (move.piece_moved.type == Piece::Type::King && move.promotion_type == Piece::Type::Rook && getDistance(move.start_pos, move.end_pos) < 2)
        {
            // Near Castle
            sf::Vector2i rook_pos = move.start_pos;
            sf::Vector2i direction = move.end_pos - move.start_pos;
            while (true)
            {
                rook_pos += direction;
                assert(GameState::checkInBoard(rook_pos));

                const Piece *piece = game_states.back().board[rook_pos.y][rook_pos.x].piece;
                if (!piece)
                    continue;

                if (piece->type == Piece::Type::Rook)
                    break;
                else
                    assert(piece->type == Piece::Type::King);
            }

            drawSprite(rook_pos.x, rook_pos.y, loadable.move_marker, sf::Color(0, 0, 255));
        }
        else if (move.is_capture)
            drawSprite(move.end_pos.x, move.end_pos.y, loadable.move_marker, sf::Color(255, 0, 0));
        else
            drawSprite(move.end_pos.x, move.end_pos.y, loadable.move_marker, sf::Color(0, 0, 255));
    }

    if (!in_place)
        drawSprite(selected_piece->pos.x, selected_piece->pos.y, loadable.move_marker, sf::Color(0, 127, 0));
}

void BoardManager::drawExtra()
{
    sf::Text text;
    text.setFont(loadable.font);
    text.setCharacterSize(text_size);
    text.setFillColor(sf::Color::Black);
    text.setPosition(text_offset);

    GameState &game_state = game_states.back();

    int player_to_move = game_state.player_white_to_move == player1_is_white ? 1 : 2;
    int player1_color = player1_is_white ? game_state.player_white_color : game_state.player_black_color;
    int player2_color = player1_is_white ? game_state.player_black_color : game_state.player_white_color;

    if (socket == NULL)
        text.setString("Player " + std::to_string(player_to_move) + " to move");
    else if (player_to_move == 1)
        text.setString("Your turn");
    else
        text.setString("Opponent turn");
    window.draw(text);

    text.setPosition(text_offset + sf::Vector2f(0, text_size));
    if (socket == NULL)
        text.setString("Player 1 Color:");
    else
        text.setString("Your Color:");
    window.draw(text);

    sf::RectangleShape square = sf::RectangleShape(sf::Vector2f(text_height, text_height));
    square.setOutlineColor(sf::Color::Black);
    square.setOutlineThickness(2);
    // square.setFillColor(colors[player1_color]);
    square.setFillColor(config.colors[player1_color]);
    square.setPosition(text.getGlobalBounds().left + text.getGlobalBounds().width + offset.x, offset.y + text_down + text_size);
    window.draw(square);

    text.setPosition(text_offset + sf::Vector2f(0, text_size * 2));
    if (socket == NULL)
        text.setString("Player 2 Color:");
    else
        text.setString("Opponent Color:");
    window.draw(text);

    // square.setFillColor(colors[player2_color]);
    square.setFillColor(config.colors[player2_color]);
    square.setPosition(text.getGlobalBounds().left + text.getGlobalBounds().width + offset.x, offset.y + text_down + text_size * 2);
    window.draw(square);

    for (int i = 0; i < 12; i++)
    {
        std::string identifier = "king" + std::to_string(i);

        sf::FloatRect rect = other_buttons[identifier].rect;
        sf::RectangleShape shape = sf::RectangleShape(sf::Vector2f(tile_size, tile_size));
        shape.setFillColor(sf::Color(config.colors[i].r, config.colors[i].g, config.colors[i].b, 127));
        shape.setPosition(rect.left, rect.top);
        window.draw(shape);

        int owner = other_buttons[identifier].active ? i : -1;

        Piece piece = Piece(sf::Vector2i(0, 0), i, owner, owner, Piece::Type::King);
        drawPiece(piece, rect);
    }

    if (other_buttons["swap0"].active)
    {
        sf::RectangleShape shape = sf::RectangleShape(sf::Vector2f(tile_size * 16, tile_size * 16));
        shape.setPosition(offset);
        shape.setFillColor(sf::Color(255, 255, 255, 127));
        window.draw(shape);

        sf::FloatRect rect = other_buttons["swap0"].rect;
        shape = sf::RectangleShape(sf::Vector2f(rect.width, rect.height));
        shape.setPosition(rect.left, rect.top);
        shape.setFillColor(sf::Color(255, 255, 255, 127));
        window.draw(shape);

        text.setString("Black");
        sf::FloatRect text_rect = text.getLocalBounds();
        text.setPosition(rect.left + (rect.width - text_rect.width) / 2, rect.top + (rect.height - text_size - text_down) / 2);
        window.draw(text);

        rect = other_buttons["swap1"].rect;
        shape = sf::RectangleShape(sf::Vector2f(rect.width, rect.height));
        shape.setPosition(rect.left, rect.top);
        shape.setFillColor(sf::Color(255, 255, 255, 127));
        window.draw(shape);

        text.setString("White");
        text_rect = text.getLocalBounds();
        text.setPosition(rect.left + (rect.width - text_rect.width) / 2, rect.top + (rect.height - text_size - text_down) / 2);
        window.draw(text);

        text.setPosition(text_offset.x, text_offset.y + text_size * 4);
        text.setString("P-2 Choose Color");
        window.draw(text);
    }

    if (!running)
    {
        assert(point == -1 || point == 1 || point == 0 || point == 2);

        text.setPosition(text_offset.x, text_offset.y + text_size * 7.5f);
        switch (point)
        {
        case -1:
        case 1:
            text.setString("Checkmate!");
            break;
        case 0:
            text.setString("Stalemate!");
            break;
        case 2:
            text.setString("Game Stopped");
            break;
        }
        window.draw(text);

        text.setPosition(text_offset.x, text_offset.y + text_size * 8.5f);
        switch (point)
        {
        case -1:
        {
            if (socket == NULL)
                text.setString("Player 2 Wins");
            else
                text.setString("You Lost");
            break;
        }
        case 0:
            text.setString("Match Draw");
            break;
        case 1:
        {
            if (socket == NULL)
                text.setString("Player 1 Wins");
            else
                text.setString("You Win");
            break;
        }
        }
        window.draw(text);
    }

    for (int i = 0; i < 5; i++)
    {
        if (!promotion_buttons[i].active)
            continue;

        PromotionButton &promotion_button = promotion_buttons[i];
        drawSquare(promotion_button.pos.x, promotion_button.pos.y + i, sf::Color(255, 255, 255, 255));

        std::array<Piece::Type, 5> piece_types = {Piece::Type::Queen, Piece::Type::Knight, Piece::Type::Rook, Piece::Type::Bishop, Piece::Type::King};

        drawPiece(Piece(promotion_button.pos + sf::Vector2i{0, i}, selected_piece->faction, selected_piece->main_owner, selected_piece->direct_owner, piece_types[i]));
    }

    {
        sf::FloatRect rect = other_buttons["exitprompt"].rect;
        sf::RectangleShape shape = sf::RectangleShape(sf::Vector2f(rect.width, rect.height));
        shape.setPosition(rect.left, rect.top);
        shape.setFillColor(sf::Color(255, 255, 255, 127));
        window.draw(shape);

        text.setString("Exit Game");
        alignText(text, rect);
        window.draw(text);

        rect = other_buttons["savegame"].rect;
        shape = sf::RectangleShape(sf::Vector2f(rect.width, rect.height));
        shape.setPosition(rect.left, rect.top);
        shape.setFillColor(sf::Color(255, 255, 255, 127));
        window.draw(shape);

        text.setString("Save Game");
        alignText(text, rect);
        window.draw(text);
    }

    if (other_buttons["exityes"].active)
    {
        sf::RectangleShape shape = sf::RectangleShape(sf::Vector2f(width, height));
        shape.setPosition(0, 0);
        shape.setFillColor(sf::Color(255, 255, 255, 127));
        window.draw(shape);

        sf::FloatRect rect = sf::FloatRect(width / 2 - text_size * 15, height / 2 - text_size * 2.5f, text_size * 30, text_size * 5);
        shape = sf::RectangleShape(sf::Vector2f(rect.width, rect.height));
        shape.setPosition(rect.left, rect.top);
        shape.setFillColor(sf::Color(127, 127, 127));
        window.draw(shape);

        rect.top = height / 2 - text_size * 2;
        rect.height = text_size * 2;
        text.setString("Are You Sure You Want To Exit?");
        alignText(text, rect);
        window.draw(text);

        rect = other_buttons["exityes"].rect;
        shape = sf::RectangleShape(sf::Vector2f(rect.width, rect.height));
        shape.setPosition(rect.left, rect.top);
        shape.setFillColor(sf::Color(255, 255, 255, 127));
        window.draw(shape);

        text.setString("Yes");
        alignText(text, rect);
        window.draw(text);

        rect = other_buttons["exitno"].rect;
        shape = sf::RectangleShape(sf::Vector2f(rect.width, rect.height));
        shape.setPosition(rect.left, rect.top);
        shape.setFillColor(sf::Color(255, 255, 255, 127));
        window.draw(shape);

        text.setString("No");
        alignText(text, rect);
        window.draw(text);
    }
}

void BoardManager::draw()
{
    drawBoard();
    drawMoves();
    drawPieces();
    drawExtra();
}

void BoardManager::isGameDone()
{
    if (legal_moves.back().size() != 0)
    {
        running = true;
        point = 2;
        return;
    }

    GameState &game_state = game_states.back();
    sf::Vector2i king_pos = game_state.player_white_to_move ? game_state.player_white_king->pos : game_state.player_black_king->pos;
    int faction = game_state.player_white_to_move ? game_state.player_white_color : game_state.player_black_color;

    if (game_state.getAllChecks(king_pos, faction).size() == 0)
    {
        running = false;
        point = 0;
    }
    else
    {
        running = false;
        point = player1_is_white == game_state.player_white_to_move ? -1 : 1;
    }
}

void BoardManager::writeToFile()
{
    int i = 0;
    while (i < 100)
    {
        std::string path = "..\\games\\game" + std::to_string(i) + ".txt";
        std::ifstream ifstream = std::ifstream(path);
        if (ifstream) // File already exists
        {
            ifstream.close();
            i++;
            continue;
        }

        std::stringstream ss;
        ss << "# This is a comment\n";
        std::string str = player1_is_white != swapped ? "P1\n" : "P2\n";
        ss << "# Started as: "
           << str;
        str = socket ? "Online\n" : "Offline\n";
        ss << "# Play mode: "
           << str;
        if (played_moves.size() > 0)
            ss << "move=" << moveToStr(played_moves[0]) << "\n";
        ss << "swapped=" << swapped << "\n";
        for (int i = 1; i < static_cast<int>(played_moves.size()); i++)
        {
            Move move = played_moves[i];
            ss << "move=" << moveToStr(move) << "\n";
        }
        switch (point)
        {
        case 0:
            str = "Draw\n";
            break;
        case 1:
            str = "P1 Win\n";
            break;
        case -1:
            str = "P2 Win\n";
            break;
        case 2:
            str = "Undecided\n";
            break;
        }
        ss << "# Result: " << str;

        std::ofstream file = std::ofstream(path);
        if (!file.is_open())
            std::cout << "Cannot Create File: " << path << std::endl;
        file << ss.str();
        file.close();
        break;
    }
}

std::string BoardManager::moveToStr(Move move)
{
    std::string str = "";
    std::array<int, 6> arr;
    arr[0] = move.start_pos.x;
    arr[1] = move.start_pos.y;
    arr[2] = move.end_pos.x;
    arr[3] = move.end_pos.y;
    arr[4] = move.promotion_type;
    arr[5] = move.piece_moved.faction;

    for (int i = 0; i < 6; i++)
    {
        if (arr[i] < 10)
            str += "0";
        str += std::to_string(arr[i]) + ",";
    }
    str = str.substr(0, str.size() - 1);
    return str;
}

Move BoardManager::strToMove(std::string string, char separator)
{
    std::array<int, 6> arr;
    std::string value = "";
    std::stringstream ss(string);
    int i = 0;
    char ch;
    while (ss >> ch)
    {
        if (ch == separator)
        {
            arr[i] = std::stoi(value);
            i++;
            value = "";
        }
        else
        {
            value += ch;
        }
    }
    if (value != "")
        arr[i] = std::stoi(value);

    Piece moved_piece = *game_states.back().board[arr[1]][arr[0]].piece;
    Piece *end_piece = game_states.back().board[arr[3]][arr[2]].piece;
    bool is_capture = end_piece && end_piece->type != Piece::Type::King;
    moved_piece.faction = arr[5];
    Move move = Move{
        sf::Vector2i(arr[0], arr[1]),
        sf::Vector2i(arr[2], arr[3]),
        moved_piece,
        is_capture,
        Piece::Type(arr[4])};

    return move;
}

void BoardManager::onPress(TileButton &button)
{
    if (!selected_piece)
    {
        selected_piece = getTile(button).piece;
        return;
    }

    if (socket != NULL && game_states.back().player_white_to_move != player1_is_white)
    {
        selected_piece = NULL;
        return;
    }

    sf::Vector2i start_pos = selected_piece->pos;
    sf::Vector2i end_pos = getTile(button).pos;
    Piece piece_moved = *selected_piece;
    bool is_capture = getTile(button).piece && start_pos != end_pos;

    assert(game_states.size() == legal_moves.size());

    Move normal_move = {start_pos, end_pos, piece_moved, is_capture};
    Move promotion_move = {start_pos, end_pos, piece_moved, is_capture, Piece::Type::Rook}; // Can also be castling
    Move king_promotion_move = {start_pos, end_pos, piece_moved, is_capture, Piece::Type::King};

    if (isMoveValid(normal_move))
    {
        doMove(normal_move);
        selected_piece = NULL;
    }
    // Pawn Promotion
    else if (isMoveValid(promotion_move) && piece_moved.type != Piece::Type::King)
    {
        for (int i = 0; i < 5; i++)
        {
            PromotionButton &promotion_button = promotion_buttons[i];
            promotion_button.active = true;
            promotion_button.pos = end_pos;
            promotion_button.rect.left = offset.x + end_pos.x * tile_size;
            promotion_button.rect.top = offset.y + (end_pos.y + i) * tile_size;
        }
        promotion_buttons[4].active = isMoveValid(king_promotion_move) ? true : false;
    }
    // Castling
    else if (piece_moved.type == Piece::Type::King && getDistance(start_pos, end_pos) > 1.5f)
    {
        if (!doMove(promotion_move))
        {
            Piece *piece = game_states.back().board[end_pos.y][end_pos.x].piece;
            std::vector<sf::Vector2i> pos_in_between = getInBetweens(start_pos, end_pos);
            // Get one distance move castle
            if (piece && piece->type == Piece::Type::Rook && pos_in_between.size() != 0)
                doMove(Move{start_pos, pos_in_between[0], piece_moved, false, Piece::Type::Rook});
        }
        selected_piece = NULL;
    }
    // Defection
    else if (isMoveValid(king_promotion_move))
    {
        doMove(king_promotion_move);
        selected_piece = NULL;
    }
    else
    {
        selected_piece = NULL;
        for (int i = 0; i < 5; i++)
        {
            promotion_buttons[i].active = false;
        }
    }
}

void BoardManager::onHold(TileButton &button)
{
}

void BoardManager::onRelease(TileButton &button)
{
}

void BoardManager::onPress(PromotionButton &button)
{
    assert(socket == NULL || game_states.back().player_white_to_move == player1_is_white);

    sf::Vector2i start_pos = selected_piece->pos;
    sf::Vector2i end_pos = getTile(button.pos).pos;
    Piece piece_moved = *selected_piece;
    bool is_capture = getTile(button.pos).piece;

    assert(game_states.size() == legal_moves.size());

    Move move = {start_pos, end_pos, piece_moved, is_capture, button.promotion_type};

    assert(doMove(move));

    selected_piece = NULL;
    for (int i = 0; i < 5; i++)
    {
        promotion_buttons[i].active = false;
    }
}

void BoardManager::onHold(PromotionButton &button)
{
}

void BoardManager::onRelease(PromotionButton &button)
{
}

void BoardManager::onPress(OtherButton &button)
{
    if (button.identifier.substr(0, 4) == "king")
        if (selected_piece)
            selected_piece = NULL;
        else
        {
            defection_piece = game_states.back().player_white_to_move ? *game_states.back().player_white_king : *game_states.back().player_black_king;
            defection_piece.faction = std::stoi(button.identifier.substr(4));
            selected_piece = &defection_piece;
        }
    else if (button.identifier.substr(0, 4) == "swap")
    {
        sf::Packet packet;
        if (button.identifier.substr(4, 1) == "0")
        {
            swap_done = true;
            swapped = false;
            refreshOtherButtons();
            packet << sf::Uint8(false);
        }
        else if (button.identifier.substr(4, 1) == "1")
        {
            swap_done = true;
            swapped = true;
            player1_is_white = !player1_is_white;
            refreshOtherButtons();
            packet << sf::Uint8(true);
        }

        if (socket != NULL)
        {
            sf::Socket::Status status = sf::Socket::Partial;
            while (status == sf::Socket::Partial)
            {
                status = socket->send(packet);
            }
            assert(status != sf::Socket::Disconnected);
            assert(status != sf::Socket::Error);
        }
    }
    else if (button.identifier.substr(0, 4) == "exit")
    {
        if (button.identifier.substr(4) == "prompt")
        {
            disableButtons();
            other_buttons["exityes"].active = true;
            other_buttons["exitno"].active = true;
        }
        else if (button.identifier.substr(4) == "yes")
        {
            disableButtons();
        }
        else if (button.identifier.substr(4) == "no")
        {
            enableButtons();
        }
    }
    else if (button.identifier == "savegame")
    {
        writeToFile();
    }
}

void BoardManager::onHold(OtherButton &button) {}

void BoardManager::onRelease(OtherButton &button) {}

void BoardManager::registerListener()
{
    using tfunc = std::function<void(TileButton & button)>;
    void (BoardManager::*top)(TileButton &button) = &BoardManager::onPress;
    void (BoardManager::*toh)(TileButton &button) = &BoardManager::onHold;
    void (BoardManager::*tor)(TileButton &button) = &BoardManager::onRelease;

    tfunc tpress = std::bind(top, this, std::placeholders::_1);
    tfunc thold = std::bind(toh, this, std::placeholders::_1);
    tfunc trelease = std::bind(tor, this, std::placeholders::_1);
    t_listener_id = ButtonEventChannel<TileButton>::registerListener(tpress, thold, trelease);

    using pfunc = std::function<void(PromotionButton & button)>;
    void (BoardManager::*pop)(PromotionButton &button) = &BoardManager::onPress;
    void (BoardManager::*poh)(PromotionButton &button) = &BoardManager::onHold;
    void (BoardManager::*por)(PromotionButton &button) = &BoardManager::onRelease;

    pfunc ppress = std::bind(pop, this, std::placeholders::_1);
    pfunc phold = std::bind(poh, this, std::placeholders::_1);
    pfunc prelease = std::bind(por, this, std::placeholders::_1);
    p_listener_id = ButtonEventChannel<PromotionButton>::registerListener(ppress, phold, prelease);

    using ofunc = std::function<void(OtherButton & button)>;
    void (BoardManager::*oop)(OtherButton &button) = &BoardManager::onPress;
    void (BoardManager::*ooh)(OtherButton &button) = &BoardManager::onHold;
    void (BoardManager::*oor)(OtherButton &button) = &BoardManager::onRelease;

    ofunc opress = std::bind(oop, this, std::placeholders::_1);
    ofunc ohold = std::bind(ooh, this, std::placeholders::_1);
    ofunc orelease = std::bind(oor, this, std::placeholders::_1);
    o_listener_id = ButtonEventChannel<OtherButton>::registerListener(opress, ohold, orelease);
}

void BoardManager::registerButtons(std::vector<Button *> &buttons)
{
    for (auto &button : tile_buttons)
    {
        buttons.push_back(&button);
    }
    for (auto &button : promotion_buttons)
    {
        buttons.push_back(&button);
    }
    for (auto it = other_buttons.begin(); it != other_buttons.end(); it++)
    {
        buttons.push_back(&it->second);
    }
}

Tile BoardManager::getTile(TileButton &button, int index)
{
    return getTile(button.pos, index);
}

Tile BoardManager::getTile(sf::Vector2i pos, int index)
{
    if (index == -1)
        index = game_states.size() - 1;
    return game_states[index].board[pos.y][pos.x];
}

bool BoardManager::isMoveValid(const Move &move, int index)
{
    if (index == -1)
        index = legal_moves.size() - 1;
    for (Move legal_move : legal_moves[index])
        if (legal_move == move)
            return true;
    return false;
}

bool BoardManager::doMove(const Move &move)
{
    if (!isMoveValid(move))
        return false;

    if (socket != NULL && player1_is_white == game_states.back().player_white_to_move)
    {
        sf::Packet packet;
        packet << move;
        sf::Socket::Status status = sf::Socket::Partial;
        while (status == sf::Socket::Partial)
        {
            status = socket->send(packet);
        }
        if (status == sf::Socket::Disconnected)
            std::cout << "Disconnected" << std::endl;
        assert(status != sf::Socket::Error);
    }

    GameState game_state = GameState(game_states.back(), move);
    game_states.push_back(std::move(game_state));
    played_moves.push_back(move);
    legal_moves.push_back(game_states.back().getMoves());

    isGameDone();
    refreshOtherButtons();

    if (move.piece_moved.type == Piece::Type::King && move.promotion_type == Piece::Type::Rook)
        sound_player.setBuffer(loadable.sounds[3]);
    else if (move.is_capture)
        sound_player.setBuffer(loadable.sounds[2]);
    else
        sound_player.setBuffer(loadable.sounds[1]);
    sound_player.play();

    return true;
}

void BoardManager::refreshOtherButtons()
{
    for (auto it = other_buttons.begin(); it != other_buttons.end(); it++)
        it->second.active = false;

    GameState &game_state = game_states.back();
    for (int i = 0; i < 12; i++)
    {
        std::string identifier = "king" + std::to_string(i);
        int ally_color = game_state.player_white_to_move ? game_state.player_white_color : game_state.player_black_color;
        other_buttons[identifier].active = game_state.getMainOwner(game_state.faction_owner[i]) == ally_color ? true : false;
    }

    if (game_states.size() == 2)
    {
        // Only show swap button to the player swapping
        if (!swap_done && (socket == NULL || player1_is_white == game_state.player_white_to_move))
        {
            other_buttons["swap0"].active = true;
            other_buttons["swap1"].active = true;

            for (auto &button : tile_buttons)
                button.active = false;
            for (int i = 0; i < 12; i++)
                other_buttons["king" + std::to_string(i)].active = false;
        }
        else
        {
            other_buttons["swap0"].active = false;
            other_buttons["swap1"].active = false;
            for (auto &button : tile_buttons)
                button.active = true;
        }
    }

    other_buttons["exitprompt"].active = true;
    other_buttons["exityes"].active = false;
    other_buttons["exitno"].active = false;
    other_buttons["savegame"].active = true;
}

void BoardManager::checkNetwork()
{
    if (socket == NULL)
        return;
    if (!running)
        return;
    assert(!socket->isBlocking());

    sf::Packet packet;
    sf::Socket::Status status = socket->receive(packet);

    if (status == sf::Socket::Disconnected)
    {
        std::cout << "Disconnected" << std::endl;
        running = false;
        return;
    }
    else if (status != sf::Socket::Done)
    {
        assert(status == sf::Socket::NotReady);
        return;
    }

    if (game_states.size() == 2 && player1_is_white != game_states.back().player_white_to_move && !swap_done)
    {
        sf::Uint8 swap;
        packet >> swap;
        ;
        if ((bool)swap)
            player1_is_white = !player1_is_white;
        swap_done = true;
    }
    else
    {
        Move move;
        packet >> move;
        assert(doMove(move));
    }
}

void BoardManager::enableButtons()
{
    for (TileButton &button : tile_buttons)
        button.active = true;

    for (PromotionButton &button : promotion_buttons)
        button.active = false;

    refreshOtherButtons();
}

void BoardManager::disableButtons()
{
    for (TileButton &button : tile_buttons)
        button.active = false;

    for (PromotionButton &button : promotion_buttons)
        button.active = false;

    for (auto it = other_buttons.begin(); it != other_buttons.end(); it++)
        it->second.active = false;
}
