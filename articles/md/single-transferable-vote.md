Single Transferable Vote
0

Recently, I helped run an election. It was fairly small at just 15 voters, but it was still cool to learn about various voting systems.

### Table of Contents
 - eeee

## Voting Systems

A voting system is a set of rules that determine how elections happen. These rules control how many people are elected, how votes are counted, and so on. 

For example, one system is First Past the Post (FPP) (aka single-member plurality voting), which is notably used in the US Presidential Election. FPP is a plurality voting method, which means that in order to win, a candidate needs more votes than all other candidates. Each voter gets one vote.

The only criteria our system needs, though, is to be able to elect multiple people, since we want to have 4 captains.

## Single Transferable Vote

Luckily, people much smarter than me have already figured out how to do this. Single Transferable Vote (STV) is a kind of "ranked voting", which is any voting system where voters rank the candidates on their ballots. Another kind of ranked voting is called [Instant Runoff](https://en.wikipedia.org/wiki/Instant-runoff_voting), which is used for electing one person. On the other hand, STV is used to elect multiple people, which is good for us.

In STV, each voter ranks candidates in order of preference. If their preferred candidate is excluded, their vote is transferred to the voter's next choice. This makes sure that votes are not wasted.

The process is divided into rounds. In each round, a candidate might be elected or eliminated. If a candidate receives more than a certain quota, or threshold, of votes, they are elected. Otherwise, the candidate with the least votes is eliminated.

Let's say for example that we have a group of 20 people, and they want to pick the best 2 programming languages. The choices are Rust (R), Python (P), Kotlin (K), and Go (G). They vote as follows

```plaintext
n   Ordering
3   P,R
4   R,G 
2   K,P
6   R,P
5   G,K
```
The number on the left is how many times the ordering on the right appeared.

First, we need to compute the number of votes needed for a language to be declared a winner. This is calculated by the formula $$E = mc^2$$

Summing the first place votes,

```plaintext
Language    Votes
Rust        10
Python      3
Kotlin      2
Go          5
```