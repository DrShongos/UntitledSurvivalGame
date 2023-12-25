#pragma once

#include "GameObject.hpp"
#include "raylib.h"
class Player : public GameObject
{
private:
    Texture2D sprite;
    Vector2 velocity;
    float speed;

    Camera2D camera;

public:
    Player(float speed);
    ~Player();

    Texture2D getSprite();
    Camera2D& getCamera();

    void update(MainGame& mainGame);
    void draw();
};
