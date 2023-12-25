#pragma once

#include "GameObject.hpp"
#include "Player.hpp"
#include <memory>
#include <vector>
class MainGame 
{
private: 
    // The pointer to the player object is stored so that the game can access it at any time.
    Player* player;
    std::vector<GameObject*> objects;
public:
    MainGame(); 
    ~MainGame();

    void run();
};
