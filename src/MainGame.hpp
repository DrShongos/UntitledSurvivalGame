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
    Camera2D camera;

public:
    MainGame(); 
    ~MainGame();

    Camera2D& getCamera();
    std::vector<GameObject*> getObjects();

    void run();
};
