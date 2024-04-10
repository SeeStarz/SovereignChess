#include "piece.h"
#include "texture.h"
#include <SFML/System/Vector2.hpp>
#include <SFML/Network/Packet.hpp>
#include <stdexcept>
#include <array>

Piece::Piece(const sf::Vector2i &pos, int faction, int main_owner, int direct_owner, Type type)
{
    this->pos = pos;
    this->faction = faction;
    this->main_owner = main_owner;
    this->direct_owner = direct_owner;
    this->type = type;
}

bool Piece::operator==(const Piece &other) const
{
    if (pos != other.pos)
        return false;
    if (faction != other.faction)
        return false;
    if (main_owner != other.main_owner)
        return false;
    if (direct_owner != other.direct_owner)
        return false;
    if (type != other.type)
        return false;
    return true;
}

bool Piece::operator!=(const Piece &other) const
{
    return !(*this == other);
}

sf::Packet &operator<<(sf::Packet &packet, const Piece &piece)
{
    return packet << (sf::Uint8)piece.pos.x << (sf::Uint8)piece.pos.y << (sf::Uint8)piece.faction << (sf::Uint8)piece.main_owner << (sf::Uint8)piece.direct_owner << (sf::Uint8)piece.type << (sf::Uint8)piece.is_alive;
}
sf::Packet &operator>>(sf::Packet &packet, Piece &piece)
{
    std::array<sf::Uint8, 7> array;
    for (int i = 0; i < 7; i++)
        packet >> array[i];
    piece.pos = sf::Vector2i(array[0], array[1]);
    piece.faction = array[2];
    piece.main_owner = array[3];
    piece.direct_owner = array[4];
    piece.type = (Piece::Type)array[5];
    piece.is_alive = array[6];
    return packet;
}