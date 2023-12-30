#pragma once

#include "GameObject.hpp"
#include "raylib.h"
class MovingObject : public GameObject
{
protected:
    Vector2 velocity;
    float speed;
public:
    MovingObject(Texture2D sprite, Vector2 position, Vector2 bounds, float speed, bitmask layer, bitmask mask);
    MovingObject(Texture2D sprite, Vector2 position, float radius, float speed, bitmask layer, bitmask mask);

    bool move(MainGame& mainGame);
};
