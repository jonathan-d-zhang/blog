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

Now for some vocab. A candidate is **hopeful** if they aren't elected and aren't eliminated. A candidate is **excluded** if they are elected or eliminated.

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

This example doesn't involve a tie, but if that were to happen, we would break the tie with a randomly decided tiebreak order.

## Header

Wow, that was annoying to calculate, huh? Luckily, I know how to program!

In my election, voters used Google Forms to submit ballots, so the data comes in CSV looking something like
```plaintext
"1","2","3","4"
"A","B","C","D"
"D","B","A","C"
...
```

This task is perfect for Pandas, so that's what we'll use. Start by doing some initialization: defining constants, loading data, etc.

```py
import math
import random
import pandas as pd

N = 4
candidates = ["A", "B", "C", "D", "E", "F"]

tiebreak_order = random.sample(candidates, k=len(candidates))

winners = []

df = pd.read_csv("data.csv")
n_voters = len(df)
df = df.assign(weight=[1] * n_voters)

quota = math.ceil(n_voters / (N + 1))

hopefuls = df["1"].value_counts()
```

We add a new column, `weight`, to track the weight of each ballot through each round. The `hopefuls` Series will contain the votes of the candidates that aren't yet elected. We'll mutate this as we move through each round. In our example, voters could only choose 2 languages, so votes could be distributed at most once. For this election though, voters select up to 4 captains, so we need to track the weight of each individual ballot.

Next we need to be able to transfer votes away from winning candidates or eliminated candidates.

```py
def transfer_ballots(away_from, eliminate):
    r = 1
    
    away_from_voters = df[str(r)] == away_from
    
    if not eliminate:
        df.loc[away_from_voters, "weight"] \
            *= surplus.loc[away_from] / hopefuls.loc[away_from]
```

First, we adjust the weight of each ballot. The new weight is just the old weight multiplied by the surplus fraction. We use `r` will keep track of what column we're currently checking.


```py
    ballots = df.loc[away_from_voters]0
    while True:
        if r == N:
            break
        
        cur_column = ballots[str(r + 1)]
        isin = cur_column.isin(hopefuls.keys())
        
        t = ballots.loc[isin].groupby(str(r + 1)).sum()
        hopefuls[t["weight"].index] += t["weight"]
        
        if isin.all():
            break

        ballots = ballots[~isin]
        r += 1
```

The two parameters, `away_from` and `eliminate`, tell us who to transfer ballots away from and whether this candidate is eliminated or elected. In our example, we didn't encounter the case where we needed to transfer a ballot, but the next candidate on the ballot was excluded.



```py
while len(winners) < N:
    greater_than_quota = hopefuls >= quota
    pending = hopefuls.loc[greater_than_quota]
    if greater_than_quota.any():
        surplus = pending - quota
        
        if len(winners) + len(pending) == N:
            winners.extend(pending.keys())

        highest = sorted(
            surplus.loc[surplus == surplus.max()].keys(), key=tiebreak_order.index
        )[0]
        
        winners.append(highest)
        transfer_ballots(highest, eliminate=False)
        hopefuls.drop(highest, inplace=True)
    else:
        lowest = sorted(
            hopefuls.loc[hopefuls == hopefuls.min()].keys(), key=tiebreak_order.index
        )[0]

        transfer_ballots(lowest, eliminate=True)
        hopefuls.drop(lowest, inplace=True)
        
        if len(winners) + len(pending) + len(hopefuls) <= N:
            winners.extend(hopefuls.keys())
```