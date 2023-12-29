#include "Collision.hpp"
#include <cmath>

static bool circleBoxCollision(Vector2& circlePosition, Collider& circle, Vector2& boxPosition, Collider& box)
{
    float testX = circlePosition.x;
    float testY = circlePosition.y; 

    if (circlePosition.x < boxPosition.x)
        testX = boxPosition.x;
    else if (circlePosition.x > boxPosition.x + box.bounds.x)
        testX = boxPosition.x + box.bounds.x;

    if (circlePosition.y < boxPosition.y)
        testY = boxPosition.y;
    else if (circlePosition.y > boxPosition.y + box.bounds.y)
        testY = boxPosition.y + box.bounds.y;

    float dx = circlePosition.x - testX;
    float dy = circlePosition.y - testY;
    float distance = std::sqrt(dx * dx + dy * dy);

    return (distance <= circle.radius);
}

bool checkCollision(Vector2& aPosition, Collider& a, Vector2& bPosition, Collider& b)
{
    if (a.type == Collider::BOUNDING_BOX && a.type == Collider::BOUNDING_BOX) {
        return (aPosition.x < bPosition.x + b.bounds.x &&
                aPosition.x + a.bounds.x > bPosition.x &&
                aPosition.y < bPosition.y + b.bounds.y &&
                aPosition.y + a.bounds.y > b.bounds.y);
    }

    if (a.type == Collider::CIRCLE && b.type == Collider::CIRCLE) {
        float dx = aPosition.x - bPosition.x;
        float dy = aPosition.y - bPosition.y;

        float distance = std::sqrt(dx * dx + dy * dy);

        return (distance <= a.radius + b.radius);
    }

    if (a.type == Collider::CIRCLE && b.type == Collider::BOUNDING_BOX)
        return circleBoxCollision(aPosition, a, bPosition, b);

    if (a.type == Collider::BOUNDING_BOX && b.type == Collider::CIRCLE)
        return circleBoxCollision(bPosition, b, aPosition, a);

    return false; // Unreachable
}
