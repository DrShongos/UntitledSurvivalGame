#pragma once

class MainGame;

#include "raylib.h"

class GameObject
{
private:
    Vector2 position;
    Vector2 bounds;

    /// When this is set to true, the object will be erased in the next frame.
    bool toDelete;
public:
    GameObject(Vector2 position, Vector2 bounds);

    Vector2& getPosition();
    Vector2& getBounds();
    bool willBeDeleted();

    void setPosition(Vector2 newPosition);

    void destroy();

    virtual void draw();
    virtual void update(MainGame& mainGame);

    bool willCollide(Vector2 pos, GameObject& other);
};
