Single Transferable Vote
0

Recently, I helped run an election. It was fairly small with just 15 voters, but it was still cool to learn about various voting systems.

### Table of Contents
 - eeee

## Voting Systems

A voting system is a set of rules that determine how elections happen. These rules control how many people are elected, how votes are counted, and so on. 

For example, one system is First Past the Post (FPP) (aka single-member plurality voting), which is notably used in the US Presidential Election. FPP is a plurality voting method, which means that in order to win, a candidate needs more votes than all other candidates. Each voter gets one vote.

The only criteria our system needs, though, is to be able to elect multiple people, since we want to have 4 captains.

## Single Transferable Vote

Luckily, people much smarter than me have already figured out how to do this. Single Transferable Vote (STV) is a kind of "ranked voting", which is any voting system where voters rank the candidates on their ballots. Another kind of ranked voting is called [Instant Runoff](https://en.wikipedia.org/wiki/Instant-runoff_voting), which is used for electing one person. On the other hand, STV is used to elect multiple people, which is good for us. The only weird thing is that the system allows fractional votes.

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

First, we need to compute the number of votes needed for a language to be declared a winner. This is calculated by the formula $$q = \lceil N / (W + 1) \rceil $$, where $$W$$ is the number of languages to select and $$N$$ is the number of voters. In our case, $$q = 7$$.

When we sum the first place votes,

```plaintext
Language    Votes
Rust        10
Python      3
Kotlin      2
Go          5
```
we find that Rust has exceeded the quota by a surplus of 3, so Rust is elected.

We now need to evenly distribute these 3 extra votes to the other languages, so they aren't wasted. The 3 surplus votes are divided amongst Rust's voters and given to their respective second choice. More specifically, each voter is given a "surplus fraction" $$s = \frac{T - q}{T}$$, where $$T$$ is the total number of votes a candidate received.

This way, it doesn't matter who actually cast the extra ballots, since the extras are evenly distributed to each of Rust's voters.

In our case, $$s = \frac{10 - 7}{7} \approx 0.4286$$. Of Rust's 10 voters, 4 voted Go second, and 6 voted Python second. So we add $$4s$$ to Go and $$6s$$ to Python. We remove Rust because Rust has already been elected. Since we elected a candidate this round, we don't need to eliminate one.

After distribution, the standings now look like

```plaintext
Language    Votes
Python      5.5714
Kotlin      2
Go          6.7143
```

None of the candidates meet the threshold, so in this round we will eliminate the candidate with the least votes, Kotlin.

Again, we must distribute the votes. This time though, the weight of the votes is not reduced, since all votes are being transferred. Both of Kotlin's voters put Python as their second choice, so we add 2 to Python. Kotlin is eliminated (😭).

The standings are now
```plaintext
Language    Votes
Python      7.5714
Go          6.7143
```

Python exceeds the threshold, so it is elected, and we've chosen our two best languages.

## Header

Wow, that was annoying to calculate, huh? Luckily, I know how to program!

In my election, voters used Google Forms to submit ballots, so the data comes in CSV looking something like
```plaintext
"1st Choice","2nd Choice","3rd Choice","4th Choice"
"A","B","C","D"
"D","B","A","C"
...
```