#include "../MainGame.hpp"
#include "MovingObject.hpp"
#include "GameObject.hpp"
#include "raylib.h"
#include "raymath.h"

MovingObject::MovingObject(Vector2 position, Vector2 bounds, float speed) : GameObject(position, bounds), velocity{0, 0}, speed{speed}
{
}

bool MovingObject::move(MainGame& mainGame)
{
    float delta = GetFrameTime();
    Vector2 movement = Vector2Scale(this->velocity, this->speed * delta);

    bool moveX = true;
    bool moveY = true;

    Vector2 xMovement = this->getPosition();
    xMovement.x += movement.x;

    Vector2 yMovement = this->getPosition();
    yMovement.y += movement.y;

    for (auto& object : mainGame.getObjects()) {
        if (object == this)
            continue;

        if (this->willCollide(xMovement, *object))
            moveX = false;

        if (this->willCollide(yMovement, *object))
            moveY = false;
    } 

    if (moveX)
        this->getPosition().x = xMovement.x;

    if (moveY)
        this->getPosition().y = yMovement.y;

    return (!moveX || !moveY);
}
