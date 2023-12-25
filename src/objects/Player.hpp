#pragma once

#include "GameObject.hpp"
#include "MovingObject.hpp"
#include "raylib.h"
class Player : public MovingObject
{
private:
    Texture2D sprite;

public:
    Player(float speed);
    ~Player();

    Texture2D& getSprite();

    void update(MainGame& mainGame);
    void draw();
};
