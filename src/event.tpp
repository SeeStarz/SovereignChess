using idType = unsigned int;

template <typename ButtonType>
using func = std::function<void(ButtonType)>;

template <typename ButtonType>
idType ButtonEventChannel<ButtonType>::registerListener(const func &press,
                                                        const func &hold, const func &release)
{
    idType id = getNextId();
    getHandler()[0][id] = press;
    getHandler()[1][id] = hold;
    getHandler()[2][id] = release;
    return id;
}

template <typename ButtonType>
void ButtonEventChannel<ButtonType>::removeListener(idType id)
{
    getHandler()[0].erase(id);
    getHandler()[1].erase(id);
    getHandler()[2].erase(id);
}

template <typename ButtonType>
void ButtonEventChannel<ButtonType>::press(ButtonType &button)
{
    for (auto it = getHandler()[0].begin(); it != getHandler()[0].end(); it++)
        it->second(button);
}

template <typename ButtonType>
void ButtonEventChannel<ButtonType>::hold(ButtonType &button)
{
    for (auto it = getHandler()[1].begin(); it != getHandler()[1].end(); it++)
        it->second(button);
}

template <typename ButtonType>
void ButtonEventChannel<ButtonType>::release(ButtonType &button)
{
    for (auto it = getHandler()[2].begin(); it != getHandler()[2].end(); it++)
        it->second(button);
}

template <typename ButtonType>
idType ButtonEventChannel<ButtonType>::getNextId()
{
    static idType id = 0;
    return id++;
}

template <typename ButtonType>
std::array<std::unordered_map<idType, std::function<void(ButtonType &)>>, 3> &ButtonEventChannel<ButtonType>::getHandler()
{
    static std::array<std::unordered_map<idType, std::function<void(ButtonType &)>>, 3> map;

    return map;
}