import { SimplePool, finalizeEvent, getPublicKey } from "nostr-tools";

export interface NostrPosition {
  pubkey: string;
  x: number;
  y: number;
}

const RELAYS = ["wss://relay.snort.social"];

export function initPool() {
  return new SimplePool();
}

export function sendPosition(pool: SimplePool, sk: Uint8Array, pos: NostrPosition) {
  const event = {
    kind: 20009,
    created_at: Math.floor(Date.now() / 1000),
    tags: [],
    content: JSON.stringify({ x: pos.x, y: pos.y }),
    pubkey: getPublicKey(sk),
  };

  const signed = finalizeEvent(event, sk);
  pool.publish(RELAYS, signed);
}

export function subscribePositions(
  pool: SimplePool,
  selfPubkey: string,
  onUpdate: (pos: NostrPosition) => void
) {
  pool.subscribeMany(
    RELAYS,
    [{ kinds: [20009] }],
    {
      onevent(event) {
        if (event.pubkey === selfPubkey) return;
        try {
          const { x, y } = JSON.parse(event.content);
          onUpdate({ pubkey: event.pubkey, x: parseFloat(x), y: parseFloat(y) });
        } catch (e) {
          console.warn("Invalid event content:", event.content);
        }
      },
    }
  );
}
