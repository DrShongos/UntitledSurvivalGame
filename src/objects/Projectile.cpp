#include "Projectile.hpp"
#include "MovingObject.hpp"
#include "raylib.h"
#include <cmath>
#include <math.h>

Projectile::Projectile(MainGame& mainGame, Vector2 direction, Vector2 position, float radius, float speed, bitmask mask) : 
    MovingObject(mainGame.getSlashSprite(), position, radius, speed, PROJECTILE_LAYER, mask), direction(direction)
{ 
}

void Projectile::update(MainGame& mainGame)
{
    rotation += 15.0f;
    this->velocity = this->direction;

    bool didCollide = this->move(mainGame);

    if (didCollide)
        this->destroy();
}
