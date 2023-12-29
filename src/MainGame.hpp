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
    std::vector<GameObject*> initializationQueue;
    Camera2D camera;
    Texture2D slashSprite; // TODO: Build an asset system and load the sprite using it.

public:
    MainGame(); 
    ~MainGame();

    Camera2D& getCamera();
    Texture2D& getSlashSprite();
    std::vector<GameObject*> getObjects();

    void run();

    template<typename ObjectType>
    ObjectType* insertObject(ObjectType* newObject)
    {
        // The objects are put into an initialization queue so that they won't interrupt the update loop and cause a segfault
        this->initializationQueue.push_back(newObject);
        TraceLog(LOG_INFO, "Queued new object for initialization");
        return newObject;
    }
};
