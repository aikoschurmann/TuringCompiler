from enum import Enum, auto
import random

class Hand(Enum):
    ROCK = 'a'
    PAPER = 'b'
    SCISSORS = 'c'

choices = {Hand.ROCK.value: "Rock", Hand.PAPER.value: "Paper", Hand.SCISSORS.value: "Scissors"}

def get_user_choice():
    while True:
        user_input = input("Enter your choice (a for Rock, b for Paper, c for Scissors): ").strip().lower()
        if user_input in choices:
            return Hand(user_input)
        else:
            print("Invalid input. Please choose a valid option.")

def determine_winner(user_hand, bot_hand):
    if user_hand == bot_hand:
        return "Draw"
    elif (user_hand == Hand.SCISSORS and bot_hand == Hand.PAPER) or \
         (user_hand == Hand.ROCK and bot_hand == Hand.SCISSORS) or \
         (user_hand == Hand.PAPER and bot_hand == Hand.ROCK):
        return "You won"
    else:
        return "You lost"

print("Welcome to Rock Paper Scissors")
print("a: Rock")
print("b: Paper")
print("c: Scissors")

user_choice = get_user_choice()
bot_choice = Hand(random.choice(list(Hand)))

print("The opponent chose", choices[bot_choice.value])
print(determine_winner(user_choice, bot_choice))
print("-------------------------")
input("Press enter to close")
