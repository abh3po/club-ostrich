import Phaser from "phaser";
import { GameScene } from "./game";
import { initPool, sendPosition, subscribePositions } from "./nostr";
import { generateSecretKey, getPublicKey } from "nostr-tools";

(async () => {
  const sk = generateSecretKey(); // Uint8Array
  const pk = getPublicKey(sk);

  const pool = initPool();

  const scene = new GameScene();
  const config: Phaser.Types.Core.GameConfig = {
    type: Phaser.AUTO,
    width: 800,
    height: 600,
    scene: [scene],
  };
  new Phaser.Game(config);

  scene.sendPosition = (x: number, y: number) => {
    sendPosition(pool, sk, { pubkey: pk, x, y });
  };

  subscribePositions(pool, pk, ({ pubkey, x, y }) => {
    scene.updateOther(pubkey, x, y);
  });
})();
