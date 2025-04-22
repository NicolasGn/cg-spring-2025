# [CodinGame Spring Challenge 2025](https://www.codingame.com/contests/spring-challenge-2025)

This is my contribution to this challenge based on the [Cephalopod](https://www.marksteeregames.com/Cephalopod_rules.pdf) dice game.

We were tasked to compute all possible final states of a 3x3 game given an initial state and a maximum depth.

Each state was represented as a 9 digits sequence where each digit was a cell of the game grid (top-left to bottom-right). The expected output was a 32 bits integer computed from the sum of all possible final states modulo 2^30.

I invested a few hours in the challenge as an opportunity to practice the Rust language I was just learning at the time.

This repo is the exact solution I submitted for the contest, as a single file as the CodinGame platform expect it.

<img src="img/score.png" alt="My score" />
