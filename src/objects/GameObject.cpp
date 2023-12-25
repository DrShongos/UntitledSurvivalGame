#include "GameObject.hpp"
#include "raylib.h"

GameObject::GameObject(Vector2 position, Vector2 bounds) : toDelete{false}
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

bool GameObject::willBeDeleted()
{
    return this->toDelete;
}

void GameObject::setPosition(Vector2 newPosition)
{
    this->position = newPosition;
}

void GameObject::destroy()
{
    TraceLog(LOG_INFO, "Object at address %p marked for deletion", this);
    this->toDelete = true;
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
        other.destroy();
        return true;
    }

    return false;
}
