#include "MainGame.hpp"
#include <raylib.h>
#include <iostream>

int main()
{
    InitWindow(1280, 720, "Untitled Survival Game");
    SetWindowState(FLAG_WINDOW_RESIZABLE);

    MainGame mainGame;
    mainGame.run();

    CloseWindow();

    return 0;
}
