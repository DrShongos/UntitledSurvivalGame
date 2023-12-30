#include "GameObject.hpp"
#include "raylib.h"
#include <cstddef>

GameObject::GameObject(Texture2D sprite, Vector2 position, Vector2 bounds, bitmask layer, bitmask mask) : 
    sprite(sprite), position(position), collider(Collider::BoundingBox(bounds, layer, mask)), toDelete(false), drawSprite(true), rotation(0.0)
{
}

GameObject::GameObject(Texture2D sprite, Vector2 position, float radius, bitmask layer, bitmask mask) :
    sprite(sprite), position(position), collider(Collider::Circle(radius, layer, mask)), toDelete(false), drawSprite(true), rotation(0.0)
{
}

GameObject::GameObject(Vector2 position, Vector2 bounds, bitmask layer, bitmask mask) : 
    position(position), collider(Collider::BoundingBox(bounds, layer, mask)), toDelete(false), drawSprite(false), rotation(0.0)
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
            DrawTextureEx(this->sprite, this->getPosition(), rotation, 3.0, WHITE);
        else {
            float offset = this->collider.radius;
            Vector2 offsetPosition = this->position;
            offsetPosition.x -= offset;
            offsetPosition.y -= offset;

            Rectangle source {0.0f, 0.0f, (float)this->sprite.width, (float)this->sprite.height};
            Rectangle dest {this->position.x - offset, this->position.y - offset, ((float)this->sprite.width) * 4.0f, ((float)this->sprite.height) * 4.0f};

            DrawTexturePro(this->sprite, source, dest, Vector2{(float)this->sprite.width / 2.0f, (float)this->sprite.height/2.0f}, this->rotation, WHITE);
        }
    } else {
        if (this->collider.type == Collider::BOUNDING_BOX)
            DrawRectangleV(this->position, this->collider.bounds, WHITE);
        else 
            DrawCircleV(this->position, this->collider.radius, WHITE);
    }
}

void GameObject::update(MainGame& mainGame)
{
}

