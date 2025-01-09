from collections import deque
import random
import constants

class SnakeGame:

    def __init__(self):

        self.width = 640
        self.height = 400
        self.direction = constants.LEFT
        self._snake_position = self._generate_random_position()
        self.fruit_position = self._generate_random_position()
        self.speed = 2
        self.snake_size = 1
        self.snake_body = deque([self._snake_position])
        self.score = 0
        self.points_per_fruit = 10
        self.movement_increment = 10
        
    def move_snake(self):
        
        x, y = self._snake_position
        
        if self.direction == constants.LEFT:
            x = (x - self.movement_increment + self.width) % self.width
        if self.direction == constants.RIGHT:
            x = (x + self.movement_increment) % self.width
        if self.direction == constants.UP:
            y = (y - self.movement_increment + self.height) % self.height
        if self.direction == constants.DOWN:
            y = (y + self.movement_increment) % self.height

        self._snake_position = (x, y)
        self.snake_body.appendleft(self._snake_position)
        self.snake_body.pop()

    def _generate_random_position(self):
        return (random.randrange(1, (self.width // 10)) * 10, random.randrange(1, (self.height // 10)) * 10)

    def is_fruit_eaten(self):
        return self.fruit_position == self._snake_position
    
    def process_fruit(self):
        self.score += self.points_per_fruit
        self.snake_body.append(tuple(self.snake_body[-1]))
        self.fruit_position = self._generate_random_position()
        self.speed += 1
    
    def is_crash(self):
        return self._snake_position in list(self.snake_body)[1:]
    
    def opposite_direction(self) -> int:
        opposites = {
            constants.LEFT: constants.RIGHT,
            constants.RIGHT: constants.LEFT,
            constants.UP: constants.DOWN,
            constants.DOWN: constants.UP
        }
        return opposites[self.direction]