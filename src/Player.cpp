#include "Player.hpp"
#include "MainGame.hpp"
#include "GameObject.hpp"
#include "raylib.h"
#include "raymath.h"

Player::Player(float speed) : GameObject(Vector2{0.0, 0.0}, Vector2{64.0, 64.0})
{
    this->sprite = LoadTexture("assets/player.png");
    this->velocity = Vector2{0.0, 0.0};
    this->speed = speed;

    this->sprite.width = 64;
    this->sprite.height = 64;

    this->camera.target = this->getPosition();
    this->camera.offset = this->getPosition();
    this->camera.zoom = 1.0f;
    this->camera.rotation = 0;
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

Camera2D& Player::getCamera()
{
    return this->camera;
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

    Vector2 nextPos = Vector2Scale(this->velocity, this->speed * delta);
    nextPos = Vector2Add(this->getPosition(), nextPos);
    
    this->getPosition() = nextPos;

    float screenWidth = (float)GetScreenWidth();
    float screenHeight = (float)GetScreenHeight();

    this->camera.offset = Vector2{screenWidth / 2.0f, screenHeight / 2.0f};
}

void Player::draw()
{
    DrawTextureV(this->sprite, this->getPosition(), WHITE);
}
