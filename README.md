near-try-vote
==================

This [React] app was initialized with [create-near-app]


Quick Start
===========
near view voting.happybits.testnet view_candidates {}
```
[
  { candidate_id: 'Tom', metadata: null, votes: 1 },
  { candidate_id: 'Bill', metadata: null, votes: 1 },
  { candidate_id: 'Peter', metadata: null, votes: 0 }
]
```

# `try-vote` will check current candidates info return by `near view voting.happybits.testnet view_candidates {}`, and how will the impact of your vote
- 'game changing vote!!'
   - near call try-vote.happybits.testnet try_vote '{"candidate_id": "Tom"}' --accountId $YOUR_ACCOUNT
- 'thanks vote'
   - near call try-vote.happybits.testnet try_vote '{"candidate_id": "Peter"}' --accountId $YOUR_ACCOUNT
- 'you can be an candidate to win'
   - if no candidates
- 'You have no choice'
   - if only one candidate
