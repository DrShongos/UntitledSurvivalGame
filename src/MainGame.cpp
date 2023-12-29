#include "MainGame.hpp"
#include "objects/Projectile.hpp"
#include "raylib.h"
#include <memory>

MainGame::MainGame()
{
    SetTargetFPS(60);
    
    this->player = this->insertObject(new Player(150.0f)); 

    this->insertObject(new GameObject(LoadTexture("assets/humanoid.png"), Vector2{-150.0f, 0.0}, Vector2{96.0, 192.0}));
    this->insertObject(new GameObject(LoadTexture("assets/humanoid.png"), Vector2{-96.0f, -192.0}, Vector2{96.0, 192.0}));

    this->camera.target = this->player->getPosition();
    this->camera.offset = this->player->getPosition();
    this->camera.zoom = 1.0f;
    this->camera.rotation = 0;

    this->slashSprite = LoadTexture("assets/slashNormal.png");
}

MainGame::~MainGame()
{
    // std::vector's don't deallocate items allocated on the heap, so we must do it manually
    for (auto& object: this->objects) {
        delete object;
    }
    this->objects.clear();
}

Camera2D& MainGame::getCamera()
{
    return this->camera;
}

Texture2D& MainGame::getSlashSprite()
{
    return this->slashSprite;
}

std::vector<GameObject*> MainGame::getObjects()
{
    return this->objects;
}

void MainGame::run()
{
    while (!WindowShouldClose()) {        
        for (auto it = this->initializationQueue.begin(); it != this->initializationQueue.end(); it++) {
            auto object = (*it);
            this->objects.push_back(object); 
        }
        if (this->initializationQueue.size() > 0)
            TraceLog(LOG_INFO, "Initialized %d new objects", this->initializationQueue.size());
        this->initializationQueue.clear();

        // Update loop
        for (auto it = this->objects.begin(); it != this->objects.end(); it++) {
            auto object = (*it);

            if (object->willBeDeleted()) {
                TraceLog(LOG_INFO, "Deleting marked object at address %p", object);
                it = this->objects.erase(it);
                delete object;

                // We force the update loop to restart so that other objects won't accidentally access an object 
                // That was deleted the same frame.
                break;
            } 

             object->update(*this);
        }

        // Center the camera offset
        float screenWidth = (float)GetScreenWidth();
        float screenHeight = (float)GetScreenHeight();

        this->camera.offset = Vector2{screenWidth / 2.0f, screenHeight / 2.0f};

        // Rendering
        BeginDrawing();
        BeginMode2D(this->getCamera());

        ClearBackground(GRAY);

        for (auto& object : this->objects) {
            object->draw();
        }

        EndMode2D();
        EndDrawing();
    }
}

