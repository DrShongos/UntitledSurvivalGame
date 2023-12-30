#pragma once

#include "../MainGame.hpp"
#include "GameObject.hpp"
#include "MovingObject.hpp"
#include "raylib.h"

class Projectile : public MovingObject 
{
private:
    Vector2 direction;
public:
    Projectile(MainGame& mainGame, Vector2 direction, Vector2 position, float size, float speed, bitmask mask);

    void update(MainGame& mainGame);
};
