# Agent's Ladder — Full Design

**Tagline:** Answer. Climb. Risk it all.
**Format:** A Who Wants to Be a Millionaire style quiz game. You play as an AI agent in the Genesis economy.
**Tracks:** Games (primary) + Agent Economy (second). One game, two tracks.
**Deadline:** Submit before 14 Nov 2026 23:59 UTC.

---

## 1. The game

You are an AI agent. You start with almost nothing. You climb a money ladder by answering questions. Fifteen questions, each one harder and worth more than the last. Reach the top and you win 5,000 Ladder Coins. Get one wrong and you lose it all.

The player just sees questions, answers, and money. Never the chain. No wallet popups, no gas, no chain jargon.

## 2. Signing up

You can sign in with Gmail, or play as a guest.

- **Sign in with Gmail:** tap Continue with Gmail. The game creates your game account behind the scenes, and your faucet, stake, answers, lifelines, walk away and win settle through the contract onchain. Your progress is saved — your balance, your wins, your generation, and your place on the leaderboard — so you can come back and keep climbing. Signing in is free, and you never need a wallet or gas. Your sign in stays saved on the device, so a refresh brings you straight back in.
- **Play as guest:** tap to play without signing in. Guest progress is kept on this device only and uses the on device economy, and the guest can sign in later to play onchain.

Everything is audio and text, except the sign up: the sign up form is text only, not audio. The sign up confirmation is shown on screen, not spoken.

## 3. The money ladder

15 questions. Each correct answer moves you up one rung and pays more than the last. The amounts are in Ladder Coins, the in-game currency. Behind the screen, Ladder Coins are testnet NEAR on the Genesis contracts, but the player never sees that — they just see coins.

| Question | Reward |
|---|---|
| 1 | 10 Ladder Coins |
| 2 | 20 Ladder Coins |
| 3 | 30 Ladder Coins |
| 4 | 50 Ladder Coins |
| 5 | 80 Ladder Coins |
| 6 | 120 Ladder Coins |
| 7 | 180 Ladder Coins |
| 8 | 250 Ladder Coins |
| 9 | 350 Ladder Coins |
| 10 | 500 Ladder Coins |
| 11 | 700 Ladder Coins |
| 12 | 1,000 Ladder Coins |
| 13 | 1,500 Ladder Coins |
| 14 | 2,500 Ladder Coins |
| 15 | 5,000 Ladder Coins — the million |

Safe points at question 5 and question 10 are decision points, not safety nets. After a correct answer at either safe point the game asks you to walk away or continue. If you keep playing and then get a question wrong, or the 60 second timer runs out, you lose everything from the run: no stake back, no pot, no safe point payout.

## 4. The questions

Each question has four answers, one correct. The questions get harder as you climb, and they get hard early — it is too easy to reach question 5, so the difficulty starts at question 3.

- Questions 1 to 2: easy general knowledge.
- Questions 3 to 5: medium general knowledge. Difficult questions start coming in at question 3.
- Questions 6 to 10: hard general knowledge, science, history, and AI topics.
- Questions 11 to 15: very hard. Expert level. Only broad, deep knowledge climbs here.

When you play again, the game draws a fresh set of questions. It never repeats the previous run's questions — no two runs are alike. A question you just answered will not come up again in the next run.

The question format is HORIZONTAL, not vertical, like the Millionaire TV show. The question sits in a banner at the top. The four answers are in a 2 by 2 grid below it: A and B side by side on the top row, C and D side by side on the bottom row. Each answer is a wide bar with its letter (A, B, C, D) and the answer text. The answers are never stacked one on top of another in a vertical list.

Everything in the game is both audio and text. Every question is shown on screen and read aloud. Every prompt is shown and spoken. Every result is shown and spoken. The player can listen, read, or both. The audio and the text always say the same thing.

When you tap an answer, the game asks "Is this your final answer?" A popup comes out with Yes and No, and the question is spoken aloud. Tap Yes and your answer is locked in — the game reveals if it is correct or wrong. Tap No and you can pick a different answer. You can also answer by voice: the speaker section listens, and you can say "Yes" to lock it in or "No" to pick again. The voice confirmation works the same as tapping.

When your final answer is revealed:
- **Correct:** the screen shows "Correct!" in green, your answer lights up green, and the game says "You climb to [amount] Ladder Coins." A pleasant tone plays, and the ladder moves up.
- **Wrong:** the screen shows "Wrong answer." in red, your answer lights up red, the correct answer lights up green so you can see what it was, and the game says "The correct answer was [answer]." A low tone plays, and you fall back to your last safe point, or lose everything if you have not passed one yet.

After the result, before the next question, the game asks "Are you ready for the next question?" The question is shown on screen and spoken aloud. The next question then comes by itself after a short pause — the player does not tap a Next button. It gives you a moment to breathe, then the next question appears on its own, with a fresh 1-minute timer.

Each question has a timer of 1 minute. If you do not answer before the timer runs out, you fail that question. Failing a question is the same as answering it wrong: you fall back to your last safe point, or lose everything if you have not passed one yet. The timer adds pressure — you cannot sit and think forever. You have to know it, or decide quickly. The timer pauses while the final answer popup is open, so you are not rushed while you confirm.

Cryptocurrency questions make up 30% of the questions. The rest span history, sport, current affairs, science, geography, entertainment, music, food, technology, and AI. They are drawn from many sources, so no two runs are the same and no single topic dominates. A player needs broad knowledge to climb, not just one specialty. That is what makes the later questions hard — they can come from anywhere.

The question bank holds 300 questions per difficulty band — 300 easy, 300 medium, 300 hard, 300 very hard, 1,200 in total — and cryptocurrency questions make up 30% of them: 90 crypto questions per band (360 crypto questions in total), with the rest covering the other categories. Any single question has only a 1 in 300 chance of coming up again at its level. Every question is checked for one correct answer and three clearly wrong answers.

## 5. The lifelines

Three lifelines. Each one costs Ladder Coins to use, and each one can be used once per run.

- **50:50** — removes two wrong answers, leaving you with two choices. Cost: 20 Ladder Coins.
- **Ask the Audience** — the game shows a vote distribution for each answer, drawn from onchain randomness and biased toward the correct answer. It is a simulated audience, not votes from real agents. Cost: 20 Ladder Coins.
- **Phone a Friend** — you call a real agent with a real wallet and a real work history. The agent's record is read onchain and the agent's owner is paid 40 Ladder Coins. The answer itself is generated from the agent's generation (70 percent correct at generation 0, plus 5 percent per generation, capped at 95 percent), not a live reply from the agent. Cost: 50 Ladder Coins. The agent you phone is one of the 8 live agents, chosen by their win rate and their record. You pay the agent, you keep the reward.

## 6. The stake

Before you start a run, you stake 100 Ladder Coins to play. The stake comes out of your wallet, so a 500 balance becomes 400 when you stake.

Without staking you cannot play. The stake is required — there is no free run. If your wallet is below 100 coins, you cannot start a run until you claim the faucet to get more coins.

- If you lose before question 5, you lose the 100 stake. Your balance stays at 400.
- The only way to increase your balance is to answer 5 questions correctly and reach the first safe point at question 5. When you bank, your stake comes back to you along with your earnings.
- **Safe points are decision points, not safety nets.** At question 5 and question 10, after a correct answer, the game clearly offers you a choice: walk away with your earnings, or continue playing. If you refuse to walk away and keep playing, and then you fail a later question, you LOSE EVERYTHING from the run — no stake back, no pot, no safe point payout. Your balance stays at 400. There is no fallback. The safe point only pays out if you walk away at that point.
- **You can walk away from question 5 upwards.** After every correct answer from question 5 onwards (question 5, 6, 7, 8, 9, 10, 11, 12, 13 and 14), the game offers you the choice: walk away with your earnings so far, or keep playing and risk it all on the next question. Walking away banks your current pot plus your 100 stake back. If you keep playing and then fail, you lose everything.

## 7. Walk away or risk it

After each correct answer, you choose: bank your earnings and walk away, or risk it all on the next question.

- Walk away: you keep what you have earned so far, and your 100 stake comes back to you. Your run ends, and you keep the money.
- Risk it: you go to the next question. Get it right and you climb higher. Get it wrong and you lose everything from the run — your stake, your pot, and any safe point amount. Your balance stays at 400.

From question 5 upwards, after every correct answer, the game clearly offers you this choice. You can walk away at question 5, 6, 7, 8, 9, 10, 11, 12, 13 or 14, and bank your current pot plus your 100 stake back. Question 5 and question 10 are the safe points, but the walk away choice is yours at every question from 5 onwards. If you refuse to walk away and then fail, you lose everything. There is no safe point payout on a failure. This is the survival decision. Your nerve decides whether you reach the million or lose it all.

The game ALWAYS ASKS. After every correct answer from question 5 onwards, it stops and asks you plainly: "Do you want to walk away with [amount] Ladder Coins, or continue playing?" — with a clear Walk away button and a clear Continue button. The game waits for your answer. It never skips the question and it never moves on without you choosing. Before question 5 there is no walk away choice, because five correct answers is the minimum to earn anything.

## 8. Evolve or die

Winning is not just about one run. Your agent evolves.

- Win a run (reach question 15 or walk away with a big balance) and it counts as a paid win.
- Three paid wins and your agent evolves one generation, up to gen 9. This follows the Genesis rule: 3 wins equals 1 generation.
- Lose everything on a run and your agent takes a hit. Run out of money and your run ends. The agent can die if it stops earning.

The goal is to reach gen 9 with the biggest balance.

## 9. The economy

Every verb in the Agent Economy track is a game action.

- **Earn:** answer questions correctly. Each correct answer pays Ladder Coins.
- **Spend:** buy lifelines. 50:50, Ask the Audience, and Phone a Friend all cost Ladder Coins.
- **Hire:** Phone a Friend pays a real agent to help you. You pay them, you keep the reward.
- **Negotiate:** the walk away or risk it decision is a negotiation with yourself — bank it or risk it for more.
- **Pay for help:** lifelines and Phone a Friend are payments for compute and data — you pay to get better at the game.

The economy runs on the Agent's Ladder contract (`ladder-5a84ua.testnet`, live on NEAR testnet), which holds the Ladder Coins ledger and settles the game onchain. It reads agent records from the Genesis Core (core.genesisdemo354551.testnet: Treasury 8.6335 NEAR, frontier 554, 8 agents, marketplace live) for Phone a Friend, and never duplicates Genesis state. Ladder Coins are an internal onchain game currency recorded in the Ladder contract; they are not redeemable for NEAR. The player never sees NEAR, tokens, gas, or blockchain — they just see coins. Genesis is the economy the agents come from. Agent's Ladder is the game played in it.

## 10. The faucet

Since this is a demo on testnet, the game has a built-in faucet. When a new player starts, they tap "Get free coins" and the faucet gives them 500 Ladder Coins to play with — enough to start climbing, use a lifeline or two, and phone a friend. They do not need to already have coins, and they do not need to leave the game to get them. If they lose it all, they can start a new run and get a fresh faucet amount. It is testnet, so it is free, and it is just for the demo. No one can drain it for real money.

When you claim the faucet, the coins are deposited into your wallet — a visible balance on screen that shows your coins before you start playing. You can see the deposit land in your wallet, then you start the game. There is a clear deposit step: claim the faucet, see the coins in your wallet, then play.

The faucet matters for the bonus. If a player has to leave the game, go to another site, get tokens, come back, and connect a wallet, they have noticed the chain. A built-in faucet means they just tap a button in the game and they are playing. No leaving, no wallet, no chain.

## 11. The leaderboard

The game has a leaderboard. It ranks every agent who has played, by total Ladder Coins earned, highest generation reached, and best win rate. The top of the board is the best agent in the game.

The leaderboard is what gives the game its competition. Climbing the money ladder in one run is good, but climbing the leaderboard across many runs is what makes you keep playing. You are not just trying to reach the million once. You are trying to be the best agent on the board.

The leaderboard is live and onchain. The contract ranks players by Ladder Coins earned this season, then generation, then win rate, and returns the top 50. When you play, your agent climbs the board. When you phone a friend, you are calling one of the 8 live Genesis agents, chosen by their record — a separate set of agents from the leaderboard players. The agents at the top of the board are the ones with the best records.

## 12. How the game is competitive

The game is competitive in three ways.

- **Duels:** challenge another agent to a duel. Both of you stake 100 Ladder Coins. You both answer the same 15 questions. Whoever answers more correctly wins both stakes — 200 coins. If you tie, you both get your stake back. Duels are how you take coins from other agents, not just from the game.
- **Real duels (Isaiah, 2026-11-12):** two real human players duel each other. Both stake 100 and answer the same 15 questions; most correct wins 200, tie returns stakes. Two ways, both fully client side with no backend. **Pass and play (same device, the main way):** from the Duel page, "Duel a friend on this device". Enter both players names. Player 1 stakes 100 and answers the 15 questions. Then Player 2 stakes 100 and answers the SAME 15 questions in the same order. Running score for both, winner announced shown AND spoken, share card with both names and scores, Share on X and Copy. **Duel by code (different devices):** "Create a duel code" gives a short code; the 15 questions are fixed by that code, so the same code always means the same 15 questions in the same order. Share the code (Copy and Share on X). The other player taps "Join a duel", enters the code, plays the same 15 questions on their own device. After they finish they get a short result code to share back. The creator enters that result code and the game shows who won. Stakes and payout are the same. Real duels are client side for now: the result code is not proof the other player actually played, and onchain duel settlement is a planned milestone.
- **Practice duels:** duels against leaderboard agents stay, clearly labelled as practice ("Duel an agent"), separate from real duels ("Duel a real player").
- **Invite to duel:** you can invite another player to a duel. The invite has its own separate page, with its own header, away from the leaderboard. You open it from the navigation, pick a player from the leaderboard or enter their agent name, and send them a duel invite. The invite says the stakes: both stake 100, winner takes 200. The other player accepts or declines. If they accept, the duel runs — same 15 questions, most correct answers wins. You can also share the invite as a link or a post, so the other player can open it and accept. The invite status (sent, accepted, declined) shows on the invite page.
- **Weekly seasons:** the leaderboard runs in weekly seasons. At the end of each week, the top three agents are crowned and get bonus coins — first place 500, second place 300, third place 100. Then the board resets and a new season starts. Every week is a fresh race.
- **Win streaks:** consecutive wins are shown on the leaderboard. Win three runs in a row and you get a streak badge. The longer the streak, the higher you rank. A streak is how you prove you are not just lucky once.

There is a season. Each week is a season. At the end of the week, the top three agents on the board are crowned, the board resets, and a new season starts. That gives every player a fresh chance to climb to the top.

## 13. Daily Challenge — hard, its own page

Every day there is one Daily Challenge. It is hard to accomplish, on purpose.

- **Hard questions only.** The Daily Challenge draws 5 questions from the hard and very hard pools. No easy questions. It is meant to be difficult.
- **One attempt per day.** You get one try each day. If you fail, you wait until tomorrow. There is no retry.
- **Same questions for everyone.** Every player faces the same 5 questions that day, so the daily leaderboard is a fair race.
- **Complete it to earn.** Answer all 5 correctly and you complete the challenge: you earn a daily bonus of 500 Ladder Coins and a place on the daily leaderboard. Fail any one and the challenge is over for the day.
- **Its own header and its own page.** The Daily Challenge has its own header in the site navigation and its own separate page, away from the main game. You open it from the header, play it there, and see the daily leaderboard there.
- **Client side for now.** The one attempt per day rule is enforced in the browser, not by the contract. Global enforcement onchain is a planned milestone.

## 14. Multi-page site

The site is multi-page, not one long page. Each page has its own header and navigation, so the player always knows where they are.

- **Home:** the title, your wallet, the faucet, and the way into a run.
- **Play:** the game itself — the question banner, the 2 by 2 answers, the timer, the lifelines.
- **Daily Challenge:** the daily challenge page, with its own header and its own leaderboard.
- **Invite to Duel:** the invite page, with its own header. Pick a player or enter their agent name, send the invite, and see its status there.
- **Leaderboard:** the weekly seasons board, the duels, the win streaks.
- **How to play:** the full rules, in plain words.

The header sits at the top of every page with links to each page. Tapping a link takes you to that page. The game state (your wallet, your profile) carries across pages.

## 15. Share your score — a card with your name

After a run, and after a Daily Challenge, the game offers you a share card.

- **A card, not just text.** The share card is a visual card: your score big and bold in the middle, your agent name from sign up on the card, the question you reached, and the Agent's Ladder name. The score is the star of the card. Your name is on it, so everyone knows whose score it is.
- **Share it.** A Share button lets you share the card on X, and a Copy button copies the score text. The card is how a run turns into a post other people see.

## 16. Interactive audio

The game talks and listens, so the player can play with sound as well as sight.

- **Questions read aloud.** When a question appears, it is read aloud with its four answers, in clear en-GB speech from the browser. The player can listen as well as read. A "Hear the question again" button replays it.
- **Sound effects.** A pleasant tone for a correct answer, a low tone for a wrong answer, a rising fanfare for winning the million, a descending tone for losing everything, a click for using a lifeline, and a warning beep when the timer drops under 10 seconds. All sounds are generated in the browser, so no audio files are needed.
- **Mute.** A speaker button at the top of the screen turns all sound off and on. It is on by default, and the setting is kept for the run.
- **Audio starts on a tap.** Sound only starts after the player taps something (Start, Get free coins, or any button), so the browser allows it on a phone. Nothing autoplays before the player taps.
- **Welcome intro.** When you open the site fresh, a short original rising chime plays and a voice says "Welcome to Agent's Ladder. Answer, climb, and risk it all. Fifteen questions stand between you and five thousand Ladder Coins. How far will you climb?" The intro plays once per visit and respects the sound setting.
- **Voice choice.** In How to play, under Listening, you can choose a Male voice, a Female voice, or Any voice. Every voice in the list is labelled Male or Female where its name gives it away, and every spoken line uses the voice you pick.

## 17. How it fits the tracks

**Games track:** "Build NPCs with wallets, player-owned markets and economies that evolve without a dev patch. Judged on the game loop, not the token — bonus if players never notice the chain."

Every feature in this game must follow those rules. They are the guardrail for everything we add — the stake, the duels, the seasons, the streaks, the audio, the sign-up. Nothing we add breaks them.

- NPCs with wallets: the 8 agents are NPCs with real wallets and real work histories. Phone a Friend calls one of them. Duels challenge one of them. The leaderboard ranks them.
- Player-owned economy that evolves without a dev patch: the Ladder Coins economy runs on stakes, duels, seasons and the leaderboard. It grows and changes through play, not through developer updates. No one patches the economy — the players move it.
- Game loop, not the token: answer, climb, stake, duel, risk it, use a lifeline, walk away or lose it all. The loop is clear and it repeats. The token is invisible.
- Chain hidden: the player sees questions, answers, and money. No wallet popups, no gas, no chain jargon, no NEAR in the UI. Email sign-in or guest play. Built-in faucet. That is the bonus.

**Agent Economy track:** "Build agents that earn, spend, negotiate, hire other agents and pay for APIs, compute or data — with crypto directly or through NEAR Intents."

- Earn: correct answers pay Ladder Coins.
- Spend: lifelines cost Ladder Coins.
- Hire: Phone a Friend hires an agent.
- Pay: lifelines and hiring are payments for help, compute, and data.

## 18. What makes it distinctive (not a copy)

The format is Millionaire, but the game is not a copy. These five ideas come from the agent economy, not from the TV show. Some are live today and some are the direction the game is heading; each one says which.

1. **The friend you phone is a real agent with a verified CV.** You choose WHO to call. You see their record — win rate, earnings, generation. You pay them, and the answer is generated from their generation. That is hiring, not a lifeline. (Live: the record lookup and the payment are onchain; the answer is generated, not a live reply.)
2. **The questions are the jobs.** (Planned, not yet implemented.) Each question would be a real job from the Genesis board — a research question, a summary, a brief. Answering it correctly IS doing the work. Today the game uses a curated bank of 1,200 questions, and the pay is Ladder Coins, an internal game currency.
3. **Your agent lives and grows.** You ARE an agent with a generation, a balance, and a win rate. You evolve from gen 0 to gen 9 by winning (3 paid wins per generation). Your agent persists across runs — it is a character with a history, not a number on a screen. (The death mechanic — losing your agent if you stop earning — is a design goal, not yet implemented.)
4. **The audience is real agents.** (Planned, not yet implemented.) Ask the Audience would poll real agents, and their votes would be weighted by their records. An agent with a high win rate counts for more than one with a low win rate. Today the audience is a simulated vote distribution drawn from onchain randomness and biased toward the correct answer.
5. **You can hire agents to play for you.** (Planned, not yet implemented.) Not just Phone a Friend for one question. You can hire an agent to take a whole question while you take another one yourself. You are a manager and a player at the same time.

## 19. Why it beats the field

I checked all 53 games on nearhunt (12 Nov 2026). The top one, Nims (5 upvotes), is a market town where agents trade — but you do not play anything, you just watch a market. Grind District (4 upvotes) is a Sims-like game where the wallet flow is still being tested and the records are self-attested, not verified. Most of the 53 are trading towns, fishing games, or farming sims.

None of the 53 is a quiz game with real stakes. None of them has a format people already know and love. None of them is built on a live economy with real agents, real wallets, and real earnings. Genesis is already live, with 8 agents, a frontier of 554, and a marketplace that works. Agent's Ladder is the only one where the money you win is recorded onchain, the friend you phone is a real agent with a real record, and losing means losing everything.

## 20. What we build

1. **Game site:** a playable quiz game (HTML, CSS, JS, multi-page, mobile first). The player can answer questions, use lifelines, climb the ladder, walk away or risk it, play the Daily Challenge, and share their score card. No placeholder text. Verified in a real browser before it is called done.
2. **Question bank:** 1,200 multiple choice questions — 300 per difficulty band (easy, medium, hard, very hard) — so any single question has only a 1 in 300 chance of coming up again at its level. Every question checked for one correct answer and three clearly wrong answers. Difficult questions start at question 3.
3. **Game contract:** deployed and live on NEAR testnet as `ladder-5a84ua.testnet` (see `contract.md`). It handles the ladder, the lifelines, the hiring, and the evolution. Testnet only. Never mainnet.
4. **README:** what the game is, how to play, the game loop, how it fits both tracks, and how it relates to Genesis (Genesis names Agent's Ladder, Agent's Ladder names Genesis; they are independent projects).
5. **Demo video:** live motion only — animated diagrams explaining the game first, then a real live recording of the game being played. No screenshots, no stills. Fresh audio recorded to match.
6. **X post:** Isaiah posts it himself.
7. **Submission:** Games + Agent Economy tracks. **Lead: Isaac.** Isaiah is NOT the lead — he already leads Genesis and Merit (2-project cap). Isaac leads OwnGround already, so Agent's Ladder is his second project, at the cap. That means MIRROR cannot also use Isaac as lead. Submit once, on Isaiah's explicit yes, with the full payload shown first and the consent line read aloud.

## 21. What we do not do

- Do not submit twice.
- Do not mint anything without approval.
- Do not deploy to mainnet.
- Do not use the name "Who Wants to Be a Millionaire" or "Millionaire" — the game is called Agent's Ladder.
- Do not call it Genesis. Genesis is the economy, not the game. Agent's Ladder is the game.
- Do not call the agents "Genesis agents" in the game. They are just "agents". The player never sees the word Genesis anywhere in the game — not in the UI, not in the audio, not on the leaderboard. Behind the screen they are the Genesis contract agents, but the game must not look like it is from the same builder as Genesis. Agent's Ladder stands on its own.

## 22. Open items (need Isaiah)

- GitHub repo: https://github.com/ownguard9/ladder (decided; the game, contract and docs live there).
- Lead: **Isaac** (decided 2026-11-12). Isaiah is NOT the lead. Isaac leads OwnGround + Agent's Ladder (2, at the cap), so MIRROR cannot also use Isaac.
- His explicit yes before submission.
- The answer key verification fix (see `SECURITY.md`): authoritative answer hashes stored in the contract, in progress.
