#include "MainGame.hpp"
#include "raylib.h"

MainGame::MainGame() : player(150.0f)
{
    SetTargetFPS(60);
}

void MainGame::run()
{
    while (!WindowShouldClose()) {        
        PollInputEvents();

        this->player.update();

        BeginDrawing();

        ClearBackground(GRAY);

        DrawTextureV(player.getSprite(), player.getPosition(), WHITE);

        EndDrawing();
    }
}
