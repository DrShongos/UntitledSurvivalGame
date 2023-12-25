#include "MainGame.hpp"
#include "GameObject.hpp"
#include "raylib.h"
#include <memory>

MainGame::MainGame()
{
    SetTargetFPS(60);
    
    this->player = new Player(150.0f);
    this->objects.push_back(this->player);

    this->objects.push_back(new GameObject(Vector2{-150.0f, 0.0f}, Vector2{96.0f, 96.0f}));

    this->camera.target = this->player->getPosition();
    this->camera.offset = this->player->getPosition();
    this->camera.zoom = 1.0f;
    this->camera.rotation = 0;
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

std::vector<GameObject*> MainGame::getObjects()
{
    return this->objects;
}

void MainGame::run()
{
    while (!WindowShouldClose()) {        
        // Update loop
        for (auto& object : this->objects) {
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
