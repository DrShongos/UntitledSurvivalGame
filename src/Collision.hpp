#pragma once

#include "raylib.h"
#include <cstdint>

typedef uint8_t bitmask;

constexpr bitmask PLAYER_LAYER = 0x1;
constexpr bitmask STATIC_OBJECT_LAYER = 0x4;
constexpr bitmask PROJECTILE_LAYER = 0x8;

struct Collider 
{
    enum {CIRCLE, BOUNDING_BOX} type;
    union 
    {
        float radius;
        Vector2 bounds;
    };

    static Collider Circle(float radius, bitmask layer, bitmask mask)
    {
        Collider collider;
        collider.type = Collider::CIRCLE;
        collider.radius = radius;
        collider.layer = layer;
        collider.mask = mask;

        return collider;
    }

    static Collider BoundingBox(Vector2 bounds, bitmask layer, bitmask mask)
    {
        Collider collider;
        collider.type = Collider::BOUNDING_BOX;
        collider.bounds = bounds;
        collider.layer = layer;
        collider.mask = mask;

        return collider;
    }

    bitmask layer;
    bitmask mask;
};

bool checkCollision(Vector2& aPosition, Collider& a, Vector2& bPosition, Collider& b);

