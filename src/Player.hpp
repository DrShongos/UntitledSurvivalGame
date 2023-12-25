#pragma once

#include "GameObject.hpp"
#include "raylib.h"
class Player : public GameObject
{
private:
    Texture2D sprite;
    Vector2 velocity;
    float speed;

public:
    Player(float speed);
    ~Player();

    Texture2D& getSprite();

    void update(MainGame& mainGame);
    void draw();
};
