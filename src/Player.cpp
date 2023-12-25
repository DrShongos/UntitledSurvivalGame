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

    //this->sprite.width = 64;
    //this->sprite.height = 64; 
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

    bool canMoveX = true;
    bool canMoveY = true;

    Vector2 nextPos = Vector2Scale(this->velocity, this->speed * delta);
    nextPos = Vector2Add(this->getPosition(), nextPos);

    Vector2 nextPosX = this->getPosition();
    nextPosX.x = nextPos.x;

    Vector2 nextPosY = this->getPosition();
    nextPosY.y = nextPos.y;

    for (auto& object : mainGame.getObjects()) {
        if (object == this)
            continue;

        if (this->willCollide(nextPosX, *object))
            canMoveX = false;

        // movement will be discarded if new position will cause the object to collide
        if (this->willCollide(nextPosY, *object))
            canMoveY = false;
    }

    if (canMoveX)
        this->getPosition().x = nextPosX.x;

    if (canMoveY)
        this->getPosition().y = nextPosY.y;
}

void Player::draw()
{
    DrawRectangleLines(this->getPosition().x, this->getPosition().y, this->getBounds().x, this->getBounds().y, RED);
    DrawTextureEx(this->sprite, this->getPosition(), 0.0, 4.0, WHITE);
}
