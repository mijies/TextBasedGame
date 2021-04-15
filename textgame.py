from abc import ABC
import re


# Game Exceptions ################

class GameOver(Exception):
    """ raised when game meets the conditions to end """


class BaseGameError(Exception):
    """ base exceptions for game errors """

    def __init__(self, *args):
        print(self.msg, *args)


class GameInputError(BaseGameError):
    msg = "Invalid Input!"


class GameDirectionError(BaseGameError):
    msg = "You can't go that way!"


class GameItemError(BaseGameError):
    msg = "Can't get"


# Globals ################

ROOMS = {
    "Point at Infinity": {
        "North": "Leonhard Euler",
        "East": "Isaac Newton",
        "South": "Carl Friedrich Gauss",
        "West": "Euclid"
    },
    "Euclid": {
        "East": "Point at Infinity"
    },
    "Isaac Newton": {
        'North': "Laplace's Demon",
        'West': "Point at Infinity"
    },
    "Leonhard Euler": {
        "East": "John von Neumann",
        "South": "Point at Infinity"
    },
    "Carl Friedrich Gauss": {
        "North": "Point at Infinity",
        "East": "Nicolas Bourbaki"
    },
    "Nicolas Bourbaki": {
        "West": "Carl Friedrich Gauss"
    },
    "John von Neumann": {
        "West": "Leonhard Euler"
    },
    "Laplace's Demon": {
        "South": "Isaac Newton"
    }
}

NUMBER_OF_ROOMS = len(ROOMS)
NUMBER_OF_ITEMS = NUMBER_OF_ROOMS - 2

START_MSG = f"""
Historical Math & Physics Text Game

Collect { NUMBER_OF_ITEMS } items to win the game, or be captured by Laplace's demon.
Move commands: go South, go North, go East, go West
Add to Inventory: get 'item name'
"""
STATUS_MSG = """
You are in the %s
Inventory : %a
"""
INPUT_MSG = f"""{ "-" * 27 }
Enter your move:
"""
WIN_MSG = """
Congratulations! You have collected all items!
Thanks for playing the game. Hope you enjoyed it.
"""
LOSS_MSG = """
NOM NOM...GAME OVER!
Thanks for playing the game. Hope you enjoyed it.
"""


# Decorators for Messages ################

def message_start(func):

    def wrapper(game):
        print(START_MSG, end='')
        func(game)

    return wrapper


def message_run(func):

    def wrapper(game):
        print(STATUS_MSG % (game.room, game.items), end='')

        if game.room.item:
            print("You see", *game.room.item)

        func(game)

    return wrapper


def message_go(func):

    def wrapper(game, direction):
        try:
            func(game, direction)

        except GameOver as e:
            print(STATUS_MSG % (game.room, game.items), end='')
            raise e

    return wrapper


def message_item(func):

    def wrapper(game, item):
        func(game, item)
        print(item, "retrieved!")

    return wrapper


# Room Definition ################

def new_room(name):
    assert isinstance(name, str)

    # extract the tailing lowercase letters of the room name
    pattern = re.search("([a-z]+)$", name).group(1)
    for key, cls in globals().items():

        if re.match('Room.+' + pattern, key):
            return cls()


class Room(ABC):

    def __str__(self):
        return self.name

    def go(self, direction):
        assert isinstance(direction, str)

        if direction in self.rooms:
            return new_room(self.rooms[direction])

        raise GameDirectionError()

    @message_item
    def consume_item(self, item):
        assert isinstance(item, str)

        if item not in self.item:
            raise GameItemError(item)

        self.item.pop()


class RoomPointAtInfinity(Room):
    name = "Point at Infinity"
    item = []
    rooms = ROOMS[name]


class RoomEuclid(Room):
    name = "Euclid"
    item = ["The Elements"]
    rooms = ROOMS[name]


class RoomNewton(Room):
    name = "Isaac Newton"
    item = ["The Principia"]
    rooms = ROOMS[name]


class RoomEuler(Room):
    name = "Leonhard Euler"
    item = ["The Opera Omnia"]
    rooms = ROOMS[name]


class RoomGauss(Room):
    name = "Carl Friedrich Gauss"
    item = ["The Disquisitiones Arithmeticae"]
    rooms = ROOMS[name]


class RoomBourbaki(Room):
    name = "Nicolas Bourbaki"
    item = ["Elements of Mathematics"]
    rooms = ROOMS[name]


class RoomNeumann(Room):
    name = "John von Neumann"
    item = ["Mathematical Foundations of Quantum Mechanics"]
    rooms = ROOMS[name]


class RoomDemon(Room):
    name = "Laplace's Demon"
    item = []
    rooms = ROOMS[name]


# Game Definition ################

class Game:

    def __init__(self):
        self.room = RoomPointAtInfinity()
        self.items = []

    @message_start
    def start(self):
        while True:
            try:
                self.run_input()

            except BaseGameError:
                pass

            except GameOver:
                break

        self.win_or_loss()

    @message_run
    def run_input(self):
        m = re.match("(go|get) (.+)", input(INPUT_MSG))

        if not m:
            raise GameInputError()

        getattr(self, m.group(1))(m.group(2))

    @message_go
    def go(self, direction):
        assert isinstance(direction, str)

        self.room = self.room.go(direction)

        if isinstance(self.room, RoomDemon):
            raise GameOver()

    def get(self, item):
        assert isinstance(item, str)

        self.room.consume_item(item)
        self.items.append(item)

        if len(self.items) is NUMBER_OF_ITEMS:
            raise GameOver()

    def win_or_loss(self):
        if len(self.items) is NUMBER_OF_ITEMS:
            print(WIN_MSG)
        else:
            print(LOSS_MSG)


if __name__ == '__main__':
    try:
        Game().start() # main() would be unnecessary

    except KeyboardInterrupt:
        print("Game cancelled")
