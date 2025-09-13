import Phaser from "phaser";

// helper function to shorten npub
function shortenNpub(pubkey: string) {
  return `npub1${pubkey.slice(5, 10)}...${pubkey.slice(-5)}`;
}

export class Player {
  sprite: Phaser.GameObjects.Image;
  label: Phaser.GameObjects.Text;
  pubkey: string;
  isSelf: boolean;
  lastX: number;

  constructor(scene: Phaser.Scene, x: number, y: number, pubkey: string, isSelf = false) {
    this.pubkey = pubkey;
    this.isSelf = isSelf;
    this.lastX = x;

    const key = isSelf ? "ostrich_player" : "ostrich_other";
    this.sprite = scene.add.image(x, y, key);
    this.sprite.setDisplaySize(isSelf ? 64 : 48, isSelf ? 64 : 48);

    this.label = scene.add
      .text(x, y - 50, shortenNpub(pubkey), {
        fontSize: "12px",
        color: "#ffffff",
        backgroundColor: "#00000088",
      })
      .setOrigin(0.5);
  }

  setPosition(x: number, y: number) {
    // Flip if moving left/right
    if (x < this.lastX) this.sprite.setFlipX(true);
    else if (x > this.lastX) this.sprite.setFlipX(false);

    this.sprite.setPosition(x, y);
    this.label.setPosition(x, y - 50);

    this.lastX = x;
  }

  get x() {
    return this.sprite.x;
  }

  get y() {
    return this.sprite.y;
  }
}
