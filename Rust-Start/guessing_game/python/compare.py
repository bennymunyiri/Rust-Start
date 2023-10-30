import random

def main():
    rand_int = random.randint(1, 10)

    flag = False
    while (flag == False):
        hello = int(input("Number: "))
        if (hello == rand_int):
            print("done wohooooo!")
            flag = True
        if (hello < rand_int):
            print("less")
            continue
        if (hello > rand_int):
            print("greate")
            continue

main()