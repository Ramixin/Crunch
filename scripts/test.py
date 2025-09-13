from crunch_lib import *

def main():
    value = input_int("Enter number count: ")
    my_list : list[Complex] = []
    backwards : int = 0
    while backwards < value:
        my_list[backwards] = input_int("enter number: ")
        backwards += 1
    disp("here is your list: ")
    disp(my_list)