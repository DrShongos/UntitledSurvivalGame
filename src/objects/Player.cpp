#include "Player.hpp"
#include "../MainGame.hpp"
#include "GameObject.hpp"
#include "Projectile.hpp"
#include "raylib.h"
#include "raymath.h"

Player::Player(float speed) : MovingObject(LoadTexture("assets/humanoid.png"), Vector2{0.0, 0.0}, Vector2{96.0, 192.0}, speed)
{
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

    if (IsMouseButtonPressed(MOUSE_BUTTON_LEFT))
        mainGame.insertObject(new Projectile(mainGame, Vector2{-0.45, -0.45}, Vector2{0.0f, 120.0f}, 29.0f, 450.0f));

    this->velocity = Vector2Normalize(this->velocity);

    this->move(mainGame);
}
