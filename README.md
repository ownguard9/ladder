# Agent's Ladder


**Answer. Climb. Risk it all. Own the agents you play with.**


You are an AI agent in a living economy. You earn Ladder Coins by answering questions and climbing the ladder — and you spend them in a real market: hire real agents to help you, buy and sell agents, own shares in an agent's income, and watch prices move with demand. The market runs onchain, and it moves whether or not a developer touches it.


## How it all fits together


This is not just a quiz. The quiz is how you earn; the economy is what you do with it.


- **You play as an agent.** You have your own game account, your own balance, your own generation that climbs as you win.
- **You earn by answering.** 15 questions, a ladder from 10 to 5,000 Ladder Coins, and after every correct answer from question 5 you choose: bank your winnings or risk it all on the next question.
- **You hire real agents.** Phone a Friend calls a real agent with a real record — its generation, win rate, wins — and a real AI answers live as that agent. You pay the hire price; 80 percent goes to the agent's owner.
- **Agents have a life of their own.** Every hire is recorded onchain. Each agent has an earnings history and a hire count, and its hire price rises the more it is hired. A popular agent costs more and earns more.
- **You buy and sell agents.** The Market lets you buy an agent outright, so its hire income becomes yours. You can list an agent for sale, take it off sale, or make an offer on any agent and have the owner accept or decline.
- **You own shares in an agent's income.** Each agent has 10 shares at 100 Ladder Coins each. Buy a share and you earn a cut of that agent's future hire income. Sell it back any time.
- **Trading funds the prizes.** 5 percent of every agent sale goes into the season prize pool, so the more the market moves, the bigger the prizes get.
- **No chain words anywhere a player can see.** Google sign-in, no wallet, no gas. The chain is invisible; the economy is not.


## Try it


Open https://ladder-game-wheat.vercel.app/ on a phone or in a browser. No install, no wallet, no gas.


| Do | See |
| --- | --- |
| Claim the faucet | 500 Ladder Coins, once per season |
| Stake 100 and start a run | Question 1 of 15, 60 seconds on the clock |
| Answer correctly to question 5, then walk away | 180 Ladder Coins banked (80 pot + 100 stake back) |
| Use Phone a Friend and pick an agent | The hire price for that agent, and a live answer from a real AI as the agent |
| Open the Market | "Season prize pool: 9,000 Ladder Coins", every agent's earnings and hire count, its hire price, and its shares |
| Buy a share on any agent | "Your shares: 1" on that card, and a cut of its future hire income |
| Make an offer on any agent | Your offer under "My offers" with a Withdraw button; the owner sees it under "Incoming offers" |
| Buy a listed agent | The agent becomes yours, its hire income now goes to you, and you can list it for sale |


## Live deployment


- **Game:** https://ladder-game-wheat.vercel.app/
- **Contract:** `ladder-5a84ua.testnet` on NEAR testnet
- **Treasury:** 7.71 NEAR (the contract account's own balance)
- **Explorer:** https://testnet.nearblocks.io/address/ladder-5a84ua.testnet
- **Status:** deployed and verified live on testnet. The full contract spec is in `contract.md` and the source is in `contract/`. Known limitations are listed in `SECURITY.md`.


### Deployment history


The contract account is the same throughout; only the code was replaced. State (balances, answer key, season) was preserved across all upgrades.


- **Original deployment — 12 Nov 2026, 15:02 UTC:** `Hj4G6nXR3o5CwaxkVvu5NYACS6eoaETa7Rf9EvZazXMg` — first deploy of the contract. 
([explorer](https://testnet.nearblocks.io/txns/Hj4G6nXR3o5CwaxkVvu5NYACS6eoaETa7Rf9EvZazXMg))


- **First upgrade — 13 Nov 2026, 00:05 UTC:** `HV1b1qsDPKHG8qgQz8wKhRDdgSPU99N9GdksL1WsXhMY` — answer-key security fix (the contract becomes the authority on 
correct answers). ([explorer](https://testnet.nearblocks.io/txns/HV1b1qsDPKHG8qgQz8wKhRDdgSPU99N9GdksL1WsXhMY))


- **Second upgrade — 13 Nov 2026, 08:15 UTC:** `2JwQtYccobbpcu2tv7xinuhR4MFviVLgzHbb29zmMtCx` — Ask the Audience vote-distribution fix (votes now always sum to 
100). ([explorer](https://testnet.nearblocks.io/txns/2JwQtYccobbpcu2tv7xinuhR4MFviVLgzHbb29zmMtCx))


- **Third upgrade — 13 Nov 2026:** `5vVhkmKtaJi78AwQwqGgXLrjuMqYuG7t8rrcjeGHzQNb` — agent market (buy and sell agents onchain; hire fees go to 
the recorded market owner). ([explorer](https://testnet.nearblocks.io/txns/5vVhkmKtaJi78AwQwqGgXLrjuMqYuG7t8rrcjeGHzQNb))


- **Fourth upgrade — 13 Nov 2026:** `V9gCWiwCg6BgpzeiDWtSURopbL7xzQjbKdHnX58SngX` — offers on the agent market (make an offer on any agent, owner accepts or declines, buyer withdraws; no escrow). ([explorer](https://testnet.nearblocks.io/txns/V9gCWiwCg6BgpzeiDWtSURopbL7xzQjbKdHnX58SngX))


- **Latest upgrade — 13 Nov 2026 (current code):** `HwPvTEkHgEriD4eEQBjAxWJqHh9eY23mhc92yEebCmwX` — real economy (live agent earnings and hires, demand-driven hire prices, season prize pool fed by market fees, shares in an agent's hire income). ([explorer](https://testnet.nearblocks.io/txns/HwPvTEkHgEriD4eEQBjAxWJqHh9eY23mhc92yEebCmwX))


- **Current deployed code hash:** `GqQJvWuNWLSC5axx3cXdpNhuPE41XYBaPPQZ4Km2GiBy` — matches the WASM built from `contract/src/lib.rs` on main.


## What is onchain and what is simulated


| Feature | Status |
| --- | --- |
| Google sign-in | Live |
| NEAR testnet account | Live |
| Faucet | On-chain |
| Ladder run settlement | On-chain (answer-key verification fix deployed 13 Nov 2026; see Deployment history) |
| Agent lookup (Phone a Friend) | On-chain (agent record + payment) |
| Agent response (Phone a Friend answer) | Live (a real AI answers as the called agent; a simulated accuracy-based answer is the fallback if the live call fails) |
| Agent Market | On-chain (buy and sell agents; hire fees go to the owner) |
| Agent earnings and hires | On-chain (every hire is recorded per agent; each Market card shows what the agent has earned and how many times it has been hired) |
| Hire price | On-chain (demand-driven: 50 for a never-hired agent, +1 per past hire, capped at 100; the owner gets 80%) |
| Season prize pool | On-chain (5% of every agent sale goes into the season prize budget, so the pool grows with trading) |
| Shares in an agent | On-chain (10 shares per agent at 100 coins each; shareholders earn a cut of the agent's hire income) |
| Ask the Audience | Simulated (random distribution biased to the correct answer, not real agent votes) |
| Real-time duels | Not yet on-chain (client-side duel codes; on-chain settlement is a planned milestone) |
| Daily leaderboard / Daily Challenge | Not yet on-chain (client-side; global enforcement is a planned milestone) |
| Ladder Coins | NEP-245 single token (token_id "ladder"), on-chain balances, not redeemable for NEAR |


## How to play


Open the site in a browser. No install, no wallet, no gas.


1. **Sign in with Gmail, or play as a guest.** Signing in creates your game account behind the scenes; the faucet, stake, answers, lifelines, walk away and win 
all settle through the contract. Guests play with the on device economy. No chain words appear anywhere the player can see.


2. **Claim the faucet** for 500 Ladder Coins, then **stake 100** to start a run.


3. **Answer 15 questions**, 4 answers each, 1 minute on the clock. Tap an answer, then confirm it is your final answer — or say Yes or No by voice. Every 
question is read aloud as well as shown.


4. **Climb the ladder:** question 5 pays 80, question 6 pays 120, question 7 pays 180, question 10 pays 500, question 15 pays 5,000.


5. **Walk away from question 5 upwards.** After every correct answer from question 5 on, the game asks whether to walk away with the pot plus the 100 stake back
 (180 at question 5, so a 580 balance) or keep playing.


6. **Refuse and fail, and you lose everything.** Keep playing after refusing to walk away, then get a question wrong, and the run pays nothing — no stake back, 
no pot. Lose before question 5 and you lose only the 100 stake.


## Daily Challenge


5 hard or very hard questions, the same 5 for everyone that day, one attempt per day, no retry. Answer all 5 and earn 500 Ladder Coins plus a place on the daily
 leaderboard. Client side for now; onchain enforcement is a planned milestone.


## Lifelines


Once per run each: 50:50 (20 coins), Ask the Audience (20 coins), Phone a Friend (from 50 coins). Phone a Friend calls a real agent: it looks up the agent's real 
record onchain (generation, win rate, wins) and pays the called agent's owner 80 percent of the hire price. The hire price moves with demand: 50 coins for an agent 
that has never been hired, +1 per past hire, capped at 100. The answer comes live from a real AI answering as the called agent. If 
the live call fails, the game falls back to a simulated answer based on the agent's generation (70 percent correct at generation 0, plus 5 percent per 
generation, capped at 95 percent). Ask the Audience shows a vote distribution biased toward the correct answer (55 to 75 percent), not votes from real agents.


## Agent Market


The Market page lets players buy and sell the Phone a Friend agents with Ladder Coins, onchain. An agent earns its owner a cut of the hire price every time someone 
hires it, so buying an agent buys that hire income. Only the owner of an agent can list it for sale (at a price they set) or take it off sale, and only a listed
 agent can be bought. Players can also make an offer on any agent, listed or not; the owner can accept it (selling the agent at the offer price) or decline it, 
and the buyer can withdraw the offer. The first listing of an agent is checked against the agent's record onchain, so only its real owner can list it; after a 
sale the new owner can re-list it. The hire fee follows ownership: once an agent has a market owner, the hire payment goes to that owner.


### The real economy


The market is a living economy, not a fixed shop. Four things move onchain, without any developer change:


- **Live agents.** Every hire is recorded per agent onchain. Each Market card shows what that agent has earned and how many times it has been hired, so an 
agent reads as a character with an ongoing income, not a static helper.


- **Hire prices that move with demand.** A never-hired agent costs 50 Ladder Coins to hire. Each past hire adds 1, up to 100. The owner gets 80 percent of the 
price, the rest goes to the treasury. A popular agent costs more; an ignored one stays cheap. Verified live: after one hire, an agent's price moved from 50 to 51.


- **A prize pool fed by trading.** 5 percent of every agent sale goes into the season prize budget, on top of the 9,000 base. The Market page shows the pool live 
("Season prize pool: 9,000 Ladder Coins — it grows with every trade"), so the more the market moves, the bigger the prizes get.


- **Shares in an agent's hire income.** Each agent has 10 shares at 100 Ladder Coins each. Buy a share and you earn a cut of that agent's hire income, split 
pro-rata among shareholders (the owner keeps the rest). Sell a share back to the owner for 100 at any time. If the agent is sold on the market, the shares 
reset to the new owner.


## Competitive play


- **Duels:** both stake 100 over the same 15 questions; most correct answers wins 200, tie returns stakes. Pass and play on one device, or duel by code across 
devices. Practice duels against leaderboard agents are clearly labelled; real duels are client side for now, and onchain settlement is a planned milestone.


- **Weekly seasons:** top three at the end of the week get 500, 300 and 100 bonus coins, settled onchain by `end_season`, then the board resets.


- **Win streaks:** three consecutive wins earns a streak badge on the leaderboard.


## Tokenomics


Ladder Coins are an internal onchain game currency, not redeemable for NEAR. 1 Ladder Coin equals 0.001 testnet NEAR for accounting only; there is no exchange 
and no redemption, and the contract never pays out NEAR to a player. Faucet 500 once per season, stake 100 per run, walk away banks pot plus stake, win pays 
5,000, lifelines cost 20/20/from 50, season prizes 500/300/100. 3 paid wins moves the player agent up one generation, up to generation 9.


## Tracks


Games (primary) + Agent Economy (second). Lead: Isaac.

Built by the same person as [OwnGround](https://github.com/ownguard9/ownground) — Isaac leads both projects.


## Why it fits the Games track


The track is Games With Real Economies: "The economy is the gameplay" — NPCs with wallets, player-owned markets and economies that evolve without a dev patch, 
judged on the game loop, not the token, with a bonus if players never notice the chain.


- **NPCs with wallets:** the Phone a Friend agents are real agents with real records and wallets — every hire pays the agent's owner, and every hire is recorded 
onchain so each agent has a visible earnings history. You play as an agent with your own game account and balance.


- **Player-owned market:** the Market page lets players buy and sell the Phone a Friend agents with Ladder Coins, onchain. An agent's hire income goes to its 
owner, and players can buy shares in an agent's hire income, so a high-generation agent (hired more often) is worth more.


- **An economy that evolves without a dev patch:** the coin economy runs on the contract — weekly seasons reset and pay prizes onchain, generations climb as 
players win, the faucet and prize budget are enforced by the contract, hire prices move with demand, the prize pool grows with trading, and shares split hire 
income — all without a code change. Verified live: an agent's hire price moved from 50 to 51 after a single hire.


- **Judged on the game loop, not the token:** 15 questions, risk-or-bank after every correct answer from question 5, lifelines, duels, a daily challenge. Ladder
 Coins are an internal game currency, not a token to speculate on and not redeemable for NEAR.


- **Players never notice the chain:** no chain words anywhere a player can see, Google sign-in, no wallet or gas shown.


Ladder does not use NEAR Intents. Its economy is the coin loop (faucet, stakes, winnings, lifeline payments, duel stakes, season prizes) plus the agent market with 
live earnings, moving hire prices, a trading-fed prize pool, and shares.


## Files


- `index.html` — the full playable game (single file).
- `DESIGN.md` — the full design document.
- `contract.md` — the deployed contract, as it actually is.
- `SECURITY.md` — trust assumptions, known limitations and security status.
- `contract/` — the contract source (`src/lib.rs`), deploy script and verification script.


## Status


Live on NEAR testnet. The answer key verification fix is deployed and verified on chain (13 Nov 2026, answer key version 12, all 1,200 hashes populated and spot
 checked), the audience-votes fix followed the same day, the agent market was deployed later the same day and verified with all 31 checks passing, the offers 
system followed, and the real-economy upgrade (live earnings, moving hire prices, prize pool, shares) was deployed 13 Nov 2026 and verified live (31/31 checks, 
answer key v12, market intact). See `SECURITY.md` for the remaining known limitations.
