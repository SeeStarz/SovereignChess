#ifndef EVENT_H
#define EVENT_H

#include <functional>
#include <array>
#include <unordered_map>

template <typename ButtonType>
class ButtonEventChannel
{
public:
    using idType = unsigned int;
    using func = std::function<void(ButtonType &button)>;

    static idType registerListener(const func &press, const func &hold, const func &release);
    static void removeListener(idType id);
    static void press(ButtonType &button);
    static void hold(ButtonType &button);
    static void release(ButtonType &button);

private:
    static idType getNextId();
    static std::array<std::unordered_map<idType, func>, 3> &getHandler();
};

#include "event.tpp"

#endif