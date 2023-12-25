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
}

MainGame::~MainGame()
{
    for (auto& object: this->objects) {
        delete object;
    }
    this->objects.clear();
}

void MainGame::run()
{
    while (!WindowShouldClose()) {        

        for (auto& object : this->objects) {
            object->update(*this);
        }

        BeginDrawing();
        BeginMode2D(this->player->getCamera());

        ClearBackground(GRAY);

        DrawTextureV(player->getSprite(), player->getPosition(), WHITE);

        for (auto& object : this->objects) {
            object->draw();
        }

        EndMode2D();
        EndDrawing();
    }
}
