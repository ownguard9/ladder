// api/phone-friend.js
// Vercel serverless function: a live AI answer for Phone a Friend.
//
// The game POSTs { question, choices, agent } and gets back
// { answerIndex (0-3), reason (one short sentence) } from a real model
// call, with the model answering as the called agent. The game falls
// back to its simulated answer whenever this function errors, so a
// failure here never breaks the game.
//
// Auth: the NEAR AI key comes from process.env.NEARAI_API_KEY. Never
// hardcode a key in this file. Set NEARAI_API_KEY in the Vercel
// dashboard (Project Settings -> Environment Variables) with a NEAR AI
// key from https://cloud.near.ai, then redeploy.

const MODEL = 'z-ai/glm-5.3-flash';
const TIMEOUT_MS = 18000;
const DEFAULT_NEARAI_URL = 'https://cloud-api.near.ai/v1/chat/completions';

// Pull { answerIndex, reason } out of the model's reply. Returns null
// when the reply is not usable, so the caller can fall back.
function parseAnswer(content) {
  if (!content || typeof content !== 'string') return null;
  let text = content.trim();
  // Strip markdown code fences if the model wrapped the JSON.
  if (text.startsWith('```')) {
    text = text.replace(/^```(?:json)?\s*/i, '').replace(/\s*```$/, '');
  }
  const start = text.indexOf('{');
  const end = text.lastIndexOf('}');
  if (start < 0 || end <= start) return null;
  let parsed;
  try {
    parsed = JSON.parse(text.slice(start, end + 1));
  } catch {
    return null;
  }
  const answerIndex = parsed.answerIndex;
  const reason = parsed.reason;
  if (!Number.isInteger(answerIndex) || answerIndex < 0 || answerIndex > 3) {
    return null;
  }
  if (typeof reason !== 'string' || !reason.trim()) return null;
  return { answerIndex, reason: reason.trim().slice(0, 200) };
}

module.exports = async (req, res) => {
  if (req.method !== 'POST') {
    res.setHeader('Allow', 'POST');
    return res.status(405).json({ error: 'method not allowed' });
  }

  const apiKey = process.env.NEARAI_API_KEY;
  if (!apiKey) {
    return res.status(500).json({ error: 'NEARAI_API_KEY not configured' });
  }

  const body = req.body || {};
  const question = body.question;
  const choices = body.choices;
  const agent = body.agent || {};
  if (typeof question !== 'string' || !question.trim()) {
    return res.status(400).json({ error: 'question is required' });
  }
  if (
    !Array.isArray(choices) ||
    choices.length !== 4 ||
    choices.some((c) => typeof c !== 'string' || !c.trim())
  ) {
    return res.status(400).json({ error: 'choices must be an array of 4 strings' });
  }
  const name = typeof agent.name === 'string' && agent.name.trim() ? agent.name.trim() : 'the agent';
  const generation = Number.isInteger(agent.generation) ? agent.generation : 0;
  const winRate = typeof agent.winRate === 'number' ? agent.winRate : 70;

  const system =
    `You are ${name}, an AI agent (generation ${generation}, win rate ${winRate}%) ` +
    'answering a quiz question in a game. Pick exactly one of the four options. ' +
    'Reply with JSON only, no other text: {"answerIndex": 0, "reason": "one short sentence"}. ' +
    'answerIndex must be 0, 1, 2 or 3.';
  const user =
    `Question: ${question.trim()}\n` +
    `A. ${choices[0]}\nB. ${choices[1]}\nC. ${choices[2]}\nD. ${choices[3]}`;

  const controller = new AbortController();
  const timer = setTimeout(() => controller.abort(), TIMEOUT_MS);
  // Overridable for local testing only; production always uses the default.
  const nearaiUrl = process.env.NEARAI_BASE_URL || DEFAULT_NEARAI_URL;
  try {
    const upstream = await fetch(nearaiUrl, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${apiKey}`,
        'x-no-aliasing': 'true',
      },
      body: JSON.stringify({
        model: MODEL,
        messages: [
          { role: 'system', content: system },
          { role: 'user', content: user },
        ],
        temperature: 0.3,
        max_tokens: 800,
      }),
      signal: controller.signal,
    });
    if (!upstream.ok) {
      return res.status(502).json({ error: 'model call failed' });
    }
    const data = await upstream.json();
    const content =
      (data.choices &&
        data.choices[0] &&
        data.choices[0].message &&
        data.choices[0].message.content) ||
      '';
    const answer = parseAnswer(content);
    if (!answer) {
      return res.status(502).json({ error: 'model returned an unusable answer' });
    }
    return res.status(200).json(answer);
  } catch (err) {
    const timedOut = err && err.name === 'AbortError';
    return res
      .status(timedOut ? 504 : 502)
      .json({ error: timedOut ? 'model call timed out' : 'model call failed' });
  } finally {
    clearTimeout(timer);
  }
};
