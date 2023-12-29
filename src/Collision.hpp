#pragma once

#include "raylib.h"

struct Collider 
{
    enum {CIRCLE, BOUNDING_BOX} type;
    union 
    {
        float radius;
        Vector2 bounds;
    };

    static Collider Circle(float radius)
    {
        Collider collider;
        collider.type = Collider::CIRCLE;
        collider.radius = radius;

        return collider;
    }

    static Collider BoundingBox(Vector2 bounds)
    {
        Collider collider;
        collider.type = Collider::BOUNDING_BOX;
        collider.bounds = bounds;

        return collider;
    }
};

bool checkCollision(Vector2& aPosition, Collider& a, Vector2& bPosition, Collider& b);

