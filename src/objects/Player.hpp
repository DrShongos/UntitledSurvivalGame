#pragma once

#include "GameObject.hpp"
#include "MovingObject.hpp"
#include "raylib.h"
class Player : public MovingObject
{
private:
public:
    Player(float speed);

    Texture2D& getSprite();

    void update(MainGame& mainGame);
};
