#include "Projectile.hpp"
#include "MovingObject.hpp"

Projectile::Projectile(MainGame& mainGame, Vector2 direction, Vector2 position, float radius, float speed) : 
    MovingObject(mainGame.getSlashSprite(), position, radius, speed), direction(direction)
{
}

void Projectile::update(MainGame& mainGame)
{
    this->velocity = this->direction;

    bool didCollide = this->move(mainGame);

    if (didCollide)
        this->destroy();
}
