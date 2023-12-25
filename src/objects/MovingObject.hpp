#pragma once

#include "GameObject.hpp"
#include "raylib.h"
class MovingObject : public GameObject
{
protected:
    Vector2 velocity;
    float speed;
public:
    MovingObject(Vector2 position, Vector2 bounds, float speed);

    bool move(MainGame& mainGame);
};
