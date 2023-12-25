#include "Player.hpp"
#include "../MainGame.hpp"
#include "GameObject.hpp"
#include "raylib.h"
#include "raymath.h"

Player::Player(float speed) : MovingObject(Vector2{0.0, 0.0}, Vector2{64.0, 64.0}, speed)
{
    this->sprite = LoadTexture("assets/player.png");
}

Player::~Player()
{
    // This line causes a segmentation fault when exiting the program.
    //UnloadTexture(this->sprite);
}

Texture2D& Player::getSprite()
{
    return this->sprite;
}


void Player::update(MainGame& mainGame)
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

    this->move(mainGame);
}

void Player::draw()
{
    DrawRectangleLines(this->getPosition().x, this->getPosition().y, this->getBounds().x, this->getBounds().y, RED);
    DrawTextureEx(this->sprite, this->getPosition(), 0.0, 4.0, WHITE);
}
