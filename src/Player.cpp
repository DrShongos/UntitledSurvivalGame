#include "Player.hpp"
#include "raylib.h"
#include "raymath.h"

Player::Player(float speed)
{
    this->sprite = LoadTexture("assets/player.png");
    this->position = Vector2{0.0, 0.0};
    this->bounds = Vector2{64.0, 64.0};
    this->velocity = Vector2{0.0, 0.0};
    this->speed = speed;

    this->sprite.width = 64;
    this->sprite.height = 64;
}

Player::~Player()
{
    // This line causes a segmentation fault when exiting the program.
    //UnloadTexture(this->sprite);
}

Texture2D Player::getSprite()
{
    return this->sprite;
}

Vector2 Player::getPosition()
{
    return this->position;
}

void Player::update()
{
    float delta = GetFrameTime();

    this->velocity = Vector2{0.0, 0.0};

    if (IsKeyDown(KEY_W))
        this->velocity.y = -1.0;

    if (IsKeyDown(KEY_S))
        this->velocity.y = 1.0;

    if (IsKeyDown(KEY_A))
        this->velocity.x = -1.0;

    if (IsKeyDown(KEY_D))
        this->velocity.x = 1.0;

    this->velocity = Vector2Normalize(this->velocity);

    Vector2 nextPos = Vector2Scale(this->velocity, this->speed * delta);
    nextPos = Vector2Add(this->position, nextPos);
    
    this->position = nextPos;
}
