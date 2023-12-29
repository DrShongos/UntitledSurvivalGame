#pragma once

class MainGame;

#include "../Collision.hpp"
#include "raylib.h"

class GameObject
{
protected:
    Texture2D sprite;
    Vector2 position;
    Collider collider;
private:
    /// When this is set to true, the object will be erased in the next frame.
    bool toDelete;
    bool drawSprite;
public:
    GameObject(Texture2D sprite, Vector2 position, Vector2 bounds);
    GameObject(Texture2D sprite, Vector2 position, float radius);
    GameObject(Vector2 position, Vector2 bounds);

    Vector2& getPosition();
    Collider& getCollider();
    bool willBeDeleted();

    void setPosition(Vector2 newPosition);

    void destroy();

    virtual void draw();
    virtual void update(MainGame& mainGame);
};
