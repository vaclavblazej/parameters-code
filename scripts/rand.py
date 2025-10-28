#!/usr/bin/env python3

import random

def rand_alphanum():
    rnd = random.randrange(0, 2*26+10)
    if rnd < 10:
        return str(rnd)
    rnd -= 10
    if rnd < 26:
        return chr(ord('a') + rnd)
    rnd -= 26
    return chr(ord('A') + rnd)


def new_id(length):
    return ''.join([rand_alphanum() for _ in range(length)])

print(new_id(6))
