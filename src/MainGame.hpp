#pragma once

#include "objects/GameObject.hpp"
#include "objects/Player.hpp"
#include <memory>
#include <vector>
class MainGame 
{
private: 
    // The pointer to the player object is stored so that the game can access it at any time.
    Player* player;
    std::vector<GameObject*> objects;
    Camera2D camera;
    Texture2D slashSprite; // TODO: Build an asset system and load the sprite using it.

public:
    MainGame(); 
    ~MainGame();

    Camera2D& getCamera();
    Texture2D& getSlashSprite();
    std::vector<GameObject*> getObjects();

    void run();
};
