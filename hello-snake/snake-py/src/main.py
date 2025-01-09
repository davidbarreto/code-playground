import pygame
import constants
from game_state import SnakeGame

# defining colors
BLACK = pygame.Color(0, 0, 0)
WHITE = pygame.Color(255, 255, 255)
RED   = pygame.Color(255, 0, 0)
GREEN = pygame.Color(0, 255, 0)
BLUE  = pygame.Color(0, 0, 255)

class App:

    def __init__(self):
        
        self._snake = SnakeGame()
        self._running = True
        self._display_surface = None
        self._fps = None
        self._size = self._snake.width, self._snake.height
        self._game_over = False
        self._key_directions = {
            pygame.K_UP: constants.UP,
            pygame.K_DOWN: constants.DOWN,
            pygame.K_LEFT: constants.LEFT,
            pygame.K_RIGHT: constants.RIGHT
        }
        
    def on_init(self):
        pygame.init()
        self._display_surface = pygame.display.set_mode(self._size, pygame.HWSURFACE | pygame.DOUBLEBUF)
        self._fps = pygame.time.Clock()
        self._font = pygame.font.Font(None, 36)
        self._running = True

    def on_event(self, event):
        
        if event.type == pygame.QUIT:
            self._running = False

        if event.type == pygame.KEYDOWN:
            if event.key in self._key_directions and self._key_directions[event.key] != self._snake.opposite_direction():
                self._snake.direction = self._key_directions[event.key]

    def on_loop(self):

        if self._snake.is_crash():
            self.game_over()
            return

        if self._snake.is_fruit_eaten():
            self._snake.process_fruit()

        self._snake.move_snake()

    def on_render(self):
        
        self._display_surface.fill(BLACK)
        
        # Draw the sanke
        for pos in self._snake.snake_body:
            pygame.draw.rect(self._display_surface, GREEN, pygame.Rect(pos[0], pos[1], 10, 10))
            
        # Draw the fruit
        pygame.draw.rect(self._display_surface, RED, pygame.Rect(self._snake.fruit_position[0], self._snake.fruit_position[1], 10, 10))
        
        # Render and display the score
        score_text = self._font.render(f"Score: {self._snake.score}", True, WHITE)
        self._display_surface.blit(score_text, (10, 10))  # Position at top-left corner
    
        pygame.display.flip()
    
    def on_cleanup(self):
        pygame.quit()

    def game_over(self):
        self._game_over = True
        self._running = False

    def on_execute(self):
        if self.on_init() == False:
            self._running = False

        while self._running:
            for event in pygame.event.get():
                self.on_event(event)
            
            self.on_loop()
            self.on_render()
            
            self._fps.tick(self._snake.speed)
            
        self.on_cleanup()

if __name__ == "__main__":
    theApp = App()
    theApp.on_execute()