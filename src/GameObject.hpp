#pragma once

class MainGame;

#include "raylib.h"

class GameObject
{
private:
    Vector2 position;
    Vector2 bounds;
public:
    GameObject(Vector2 position, Vector2 bounds);

    Vector2& getPosition();
    Vector2& getBounds();

    virtual void draw();
    virtual void update(MainGame& mainGame);
};
