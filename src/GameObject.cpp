#include "GameObject.hpp"
#include "raylib.h"

GameObject::GameObject(Vector2 position, Vector2 bounds)
{
    this->position = position;
    this->bounds = bounds;
}


Vector2& GameObject::getPosition()
{
    return this->position;
}

Vector2& GameObject::getBounds()
{
    return this->bounds;
}

void GameObject::setPosition(Vector2 newPosition)
{
    this->position = newPosition;
}

void GameObject::draw()
{
    DrawRectangleV(this->position, this->bounds, WHITE);
}

void GameObject::update(MainGame& mainGame)
{
}

bool GameObject::willCollide(Vector2 pos, GameObject& other)
{
    if (pos.x < other.position.x + other.bounds.x && 
        pos.x + this->bounds.x > other.position.x &&
        pos.y < other.position.y + other.bounds.y &&
        pos.y + this->bounds.y > other.position.y) {
        return true;
    }

    return false;
}
