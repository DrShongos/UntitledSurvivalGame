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

void GameObject::draw()
{
    DrawRectangleV(this->position, this->bounds, WHITE);
}

void GameObject::update(MainGame& mainGame)
{
}
