#include "GameObject.hpp"
#include "raylib.h"
#include <cstddef>

GameObject::GameObject(Texture2D sprite, Vector2 position, Vector2 bounds) : 
    sprite(sprite), position(position), collider(Collider::BoundingBox(bounds)), toDelete(false), drawSprite(true)
{
}

GameObject::GameObject(Texture2D sprite, Vector2 position, float radius) :
    sprite(sprite), position(position), collider(Collider::Circle(radius)), toDelete(false), drawSprite(true)
{
}

GameObject::GameObject(Vector2 position, Vector2 bounds) : 
    position(position), collider(Collider::BoundingBox(bounds)), toDelete(false), drawSprite(false)
{
}


Vector2& GameObject::getPosition()
{
    return this->position;
}

Collider& GameObject::getCollider()
{
    return this->collider;
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
    if (this->collider.type == Collider::BOUNDING_BOX)
        DrawRectangleLines(this->getPosition().x, this->getPosition().y, this->collider.bounds.x, this->collider.bounds.y, RED);
    else if (this->collider.type == Collider::CIRCLE)
        DrawCircleLinesV(this->position, this->collider.radius, RED);

    if (this->drawSprite) {
        if (this->collider.type == Collider::BOUNDING_BOX)
            DrawTextureEx(this->sprite, this->getPosition(), 0.0, 3.0, WHITE);
        else {
            float offset = this->collider.radius;
            Vector2 offsetPosition = this->position;
            offsetPosition.x -= offset;
            offsetPosition.y -= offset;
            DrawTextureEx(this->sprite, offsetPosition, 0.0, 4.0, WHITE);
        }

    }
    else {
        if (this->collider.type == Collider::BOUNDING_BOX)
            DrawRectangleV(this->position, this->collider.bounds, WHITE);
        else 
            DrawCircleV(this->position, this->collider.radius, WHITE);
    }
}

void GameObject::update(MainGame& mainGame)
{
}

