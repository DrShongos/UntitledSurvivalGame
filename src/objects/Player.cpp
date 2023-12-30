#include "Player.hpp"
#include "../MainGame.hpp"
#include "GameObject.hpp"
#include "Projectile.hpp"
#include "raylib.h"
#include "raymath.h"
#include <cmath>

Player::Player(float speed) : MovingObject(LoadTexture("assets/humanoid.png"), Vector2{0.0, 0.0}, Vector2{96.0, 192.0}, speed, PLAYER_LAYER, STATIC_OBJECT_LAYER | PROJECTILE_LAYER)
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

    this->velocity = Vector2Normalize(this->velocity);

    this->move(mainGame);

    if (IsMouseButtonPressed(MOUSE_BUTTON_LEFT)) {
        Vector2 mousePos = GetMousePosition();
        mousePos = GetScreenToWorld2D(mousePos, mainGame.getCamera());

        Vector2 direction = Vector2Normalize(Vector2Subtract(mousePos, this->position));

        Vector2 offsetDir = Vector2AddValue(direction, 1.0);

        Vector2 offset = Vector2{
            this->position.x + (this->collider.bounds.x * 0.5f * offsetDir.x),
            this->position.y + (this->collider.bounds.y * 0.5f * offsetDir.y)
        };
        //Vector2Scale(offset, 0.05);

        mainGame.insertObject(new Projectile(mainGame, direction, offset, 29.0f, 450.0f, STATIC_OBJECT_LAYER));
    }
}
