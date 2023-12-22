#pragma once

#include "raylib.h"
class Player 
{
private:
    Texture2D sprite;
    Vector2 position;
    Vector2 bounds;
    Vector2 velocity;
    float speed;

public:
    Player() {}; 
    Player(float speed);
    ~Player();

    Texture2D getSprite();
    Vector2 getPosition();

    void update();
};
